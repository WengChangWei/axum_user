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
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use entitys::{prelude::*, users_demo::Model, *};
use time_library::Timestamp;
// use validator::Validate;
// use thiserror::Error;

const DATABASE_URL: &str = "mysql://root:123456@localhost:3306/rust_demo";

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(root))
    .route("/users", post(create_user).put(update_user))
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
        Err(e) => {
            e.to_string().into_response()
        },
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
            Err(DbErr::RecordNotFound("not found".to_string()))
        },
    }
}


async fn update_user(Json(user): Json<User>) -> Result<String, AppError> {
    do_update_user(user).await;
    Ok("Ok".to_string())
    // match res {
    //     Ok(_) => {
    //         Ok(Json(user))
    //     },
    //     Err(e) => {
    //         e.to_string().into_response()
    //     },
    // }
    
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

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
}

// The kinds of errors we can hit in our application.
enum AppError {
    // The request body contained invalid JSON
    JsonRejection(JsonRejection),
    // Some error from a third party library we're using
    TimeError(time_library::Error),
}

// Tell axum how `AppError` should be converted into a response.
//
// This is also a convenient place to log errors.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            AppError::JsonRejection(rejection) => {
                // This error is caused by bad user input so don't log it
                (rejection.status(), rejection.body_text())
            }
            AppError::TimeError(err) => {
                // Because `TraceLayer` wraps each request in a span that contains the request
                // method, uri, etc we don't need to include those details here
                tracing::error!(%err, "error from time_library");

                // Don't expose any details about the error to the client
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong".to_owned(),
                )
            }
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}

impl From<time_library::Error> for AppError {
    fn from(error: time_library::Error) -> Self {
        Self::TimeError(error)
    }
}

mod time_library {
    use std::sync::atomic::{AtomicU64, Ordering};

    use serde::Serialize;

    #[derive(Serialize, Clone)]
    pub struct Timestamp(u64);

    impl Timestamp {
        pub fn now() -> Result<Self, Error> {
            static COUNTER: AtomicU64 = AtomicU64::new(0);

            // Fail on every third call just to simulate errors
            if COUNTER.fetch_add(1, Ordering::SeqCst) % 3 == 0 {
                Err(Error::FailedToGetTime)
            } else {
                Ok(Self(1337))
            }
        }
    }

    #[derive(Debug)]
    pub enum Error {
        FailedToGetTime,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "failed to get time")
        }
    }
}