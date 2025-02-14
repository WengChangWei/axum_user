mod entitys;

use axum::{
    body::Body, 
    extract::{rejection::JsonRejection, Path}, // rejection::FormRejection, Form, FromRequest, Request,
    http::StatusCode, 
    response::{IntoResponse, Response}, 
    routing::{get, post, put}, 
    Json, 
    Router
};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use entitys::{prelude::*, users_demo::Model, *};
// use validator::Validate;
// use thiserror::Error;

const DATABASE_URL: &str = "mysql://root:123456@localhost:3306";
const DB_NAME: &str = "rust_demo";

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(root))
    .route("/users", post(create_user).put(update_user).delete(delete_user))
    .route("/users/:username", get(get_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn connect_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let db = match db.get_database_backend() {
        DatabaseBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(), 
                format!("CREATE DATABASE IF NOT EXISTS `{}`", DB_NAME),
            )).await?;
            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        },
        _ => {db}
    };

    Ok(db)
}

async fn root() -> Response<Body> {
    "Hello, World!".into_response()
}

async fn create_user(Json(user): Json<User>) -> Result<String, AppError> {
    do_create_user(user).await.map_err(|e| AppError::DatabaseError(e))?;
    Ok("create user success!".to_string())
}
async fn do_create_user(user: User) -> Result<(), DbErr> {
    let db = connect_db().await?;
    let user_model: users_demo::ActiveModel = users_demo::ActiveModel {
            id: ActiveValue::NotSet,
            username: ActiveValue::Set(user.username),
            email: ActiveValue::Set(user.email),
            ..Default::default()
        };
    let res = UsersDemo::insert(user_model).exec(&db).await;
    match res {
        Ok(_) => {
            Ok(())
        },
        Err(e) => {
            Err(e)
        },
    }

}

async fn get_user(Path(name): Path<String>) -> Result<String, AppError> {
    let user_model = do_get_user(name).await.map_err(|e| AppError::DatabaseError(e))?;
    let user = User {
        username: user_model.username,
        email: user_model.email
    };
    Ok(serde_json::to_string(&user).unwrap())
}

async fn do_get_user(username: String) -> Result<Model, DbErr> {
    let db = connect_db().await?;
    let user = UsersDemo::find()
        .filter(users_demo::Column::Username.eq(username))
        .one(&db)
        .await?;
    match user {
        Some(s) => {
            return Ok(s)
        },
        None => {
            Err(DbErr::RecordNotFound("user not found".to_string()))
        },
    }
}


async fn update_user(Json(user): Json<User>) -> Result<String, AppError> {
    do_update_user(user).await.map_err(|e| AppError::DatabaseError(e))?;
    Ok("update user success!".to_string())
}

async fn do_update_user(user: User) -> Result<(), DbErr> {
    let username = user.username;
    let user_model = do_get_user(username).await;
    let db = connect_db().await?;
    match user_model {
        Ok(model) => {
            let db_model = users_demo::ActiveModel {
                            id: ActiveValue::Set(model.id),
                            username: ActiveValue::Set(model.username),
                            email: ActiveValue::Set(user.email),
                            ..Default::default()
                        };
            db_model.update(&db).await?;
            Ok(())
        },
        Err(e) => {
            Err(e)
        },
    }
}

async fn delete_user(Json(user): Json<User>) -> Result<String, AppError> {
    do_delete_user(user).await.map_err(|e| AppError::DatabaseError(e))?;
    Ok("delete user success!".to_string())
}

async fn do_delete_user(user: User) -> Result<(), DbErr> {
    let db = connect_db().await?;
    let user_demo = do_get_user(user.username).await;
    match user_demo {
        Ok(demo) => {
            let user_model = users_demo::ActiveModel {
                id: ActiveValue::Set(demo.id),
                username: ActiveValue::Set(demo.username),
                ..Default::default()
            };
        user_model.delete(&db).await?;
        },
        Err(e) => {
            return Err(e)
        },
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
}

enum AppError {
    DatabaseError(sea_orm::DbErr),
}


impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            AppError::DatabaseError(err) => {
                tracing::error!(%err, "Database error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    err.to_string(),
                )
            }
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        Self::DatabaseError(err)
    }
}