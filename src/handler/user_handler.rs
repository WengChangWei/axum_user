

use axum::{body::Body, extract::{Path, State}, http::StatusCode, response::{IntoResponse, Response}, Json};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use crate::{boot_server::state::AppState, entitys::{prelude::*, users_demo::Model, *}};

pub struct UserHandler;

impl UserHandler {
    
    pub async fn root() -> Response<Body> {
        "Hello, World!".into_response()
    }
    
    pub async fn create_user(
        State(state): State<AppState>,
        Json(user): Json<User>
    ) -> Result<String, AppError> {
        Self::do_create_user(state,user).await.map_err(|e| AppError::DatabaseError(e))?;
        Ok("create user success!".to_string())
    }
    async fn do_create_user(
        state: AppState, 
        user: User
    ) -> Result<(), DbErr> {
        let db = state.conn;
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
    
    pub async fn get_user(
        State(state): State<AppState>,
        Path(name): Path<String>
    ) -> Result<String, AppError> {
        let user_model = Self::do_get_user(state, name).await.map_err(|e| AppError::DatabaseError(e))?;
        let user = User {
            username: user_model.username,
            email: user_model.email
        };
        Ok(serde_json::to_string(&user).unwrap())
    }
    
    async fn do_get_user(
        state: AppState, 
        username: String
    ) -> Result<Model, DbErr> {
        let db = state.conn;
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
    
    
    pub async fn update_user(
        State(state): State<AppState>,
        Json(user): Json<User>
    ) -> Result<String, AppError> {
        Self::do_update_user(state, user).await.map_err(|e| AppError::DatabaseError(e))?;
        Ok("update user success!".to_string())
    }
    
    async fn do_update_user(
        state: AppState,
        user: User
    ) -> Result<(), DbErr> {
        let username = user.username;
        let conn = state.clone();
        let user_model = Self::do_get_user(conn, username).await;
        let conn = state.clone();
        let db = conn.conn;
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
    
    pub async fn delete_user(
        State(state): State<AppState>,
        Json(user): Json<User>
    ) -> Result<String, AppError> {
        Self::do_delete_user(state, user).await.map_err(|e| AppError::DatabaseError(e))?;
        Ok("delete user success!".to_string())
    }
    
    async fn do_delete_user(
        state: AppState,
        user: User
    ) -> Result<(), DbErr> {
        let conn = state.clone();
        let user_demo = Self::do_get_user(conn, user.username).await;
        let conn = state.clone();
        let db = conn.conn;
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
    
}

#[derive(Serialize, Deserialize)]
pub struct User {
    username: String,
    email: String,
}

pub enum AppError {
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