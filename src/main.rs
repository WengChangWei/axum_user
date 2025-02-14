mod entitys;

use axum::{
    body::Body, 
    extract::{Path}, // rejection::FormRejection, Form, FromRequest, Request,
    http::StatusCode, 
    response::{IntoResponse, Response}, 
    routing::{get, post}, 
    Json, 
    Router
};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use entitys::{prelude::*, users_demo::Model, *};
// , de::DeserializeOwned
// use validator::Validate;
// use thiserror::Error;

const DATABASE_URL: &str = "mysql://root:123456@localhost:3306/rust_demo";

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(root))
    .route("/users", post(create_user))
    .route("/users/:username", get(get_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn connect_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    Ok(db)
}

async fn root() -> Response<Body> {
    "Hello, World!".into_response()
}

async fn create_user() -> Response<Body> {
    let res = do_create_user().await;
    match res {
        Ok(_) => {"Ok".into_response()},
        Err(e) => {
            let err_msg = format!("Failed:{}", e);
            err_msg.into_response()
        },
    }
}
async fn do_create_user() -> Result<(), DbErr> {
    let db = connect_db().await?;
    let user_model: users_demo::ActiveModel = users_demo::ActiveModel {
            id: ActiveValue::NotSet,
            username: ActiveValue::Set("test".to_string()),
            email: ActiveValue::Set("123@qq.com".to_string()),
            ..Default::default()
        };
    let _res: InsertResult<users_demo::ActiveModel> = UsersDemo::insert(user_model).exec(&db).await?;
    Ok(())
}

async fn get_user(Path(name): Path<String>) -> Response<Body> {
    let user = do_get_user(name).await;
    match user {
        Ok(user) => {user.username.into_response()},
        Err(_) => {"No found".into_response()},
    }
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
            panic!("UserName is not found")
        },
    }
}

// async fn update_user(ValidatedForm(name): ValidatedForm<String>) -> (StatusCode, Json<User>) {
//     let user = User {
//         id: 1337,
//         username: name,
//     };

//     (StatusCode::OK, Json(user))
// }

// #[derive(Debug, Clone, Copy, Default)]
// pub struct ValidatedForm<T>(pub T);

// impl<T, S> FromRequest<S> for ValidatedForm<T>
// where
//     T: DeserializeOwned + Validate,
//     S: Send + Sync,
//     Form<T>: FromRequest<S, Rejection = FormRejection>,
// {
//     type Rejection = ServerError;

//     async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
//         let Form(value) = Form::<T>::from_request(req, state).await?;
//         value.validate()?;
//         Ok(ValidatedForm(value))
//     }
// }

// #[derive(Debug, Error)]
// pub enum ServerError {
//     #[error(transparent)]
//     ValidationError(#[from] validator::ValidationErrors),

//     #[error(transparent)]
//     AxumFormRejection(#[from] FormRejection),
// }

// impl IntoResponse for ServerError {
//     fn into_response(self) -> Response {
//         match self {
//             ServerError::ValidationError(_) => {
//                 let message = format!("Input validation error: [{self}]").replace('\n', ", ");
//                 (StatusCode::BAD_REQUEST, message)
//             }
//             ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
//         }
//         .into_response()
//     }
// }



#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}