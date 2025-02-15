

use axum::{body::Body, extract::{Path, State}, http::StatusCode, response::{IntoResponse, Response}, Json};

use serde::Serialize;
use crate::{
    boot_server::state::AppState, 
    service::user_service::UserService, vo::user::User,
};

pub struct UserHandler;

impl UserHandler {
    
    pub async fn root() -> Response<Body> {
        "Hello, World!".into_response()
    }
    
    pub async fn create_user(
        State(state): State<AppState>,
        Json(user): Json<User>
    ) -> Result<String, AppError> {
        UserService::do_create_user(&state,user).await.map_err(|e| AppError::DatabaseError(e))?;
        Ok("create user success!".to_string())
    }

    
    pub async fn get_user(
        State(state): State<AppState>,
        Path(name): Path<String>
    ) -> Result<String, AppError> {
        let user_model = UserService::do_get_user(&state, name).await.map_err(|e| AppError::DatabaseError(e))?;
        let user = User {
            username: user_model.username,
            email: user_model.email
        };
        Ok(serde_json::to_string(&user).unwrap())
    }
    
    
    pub async fn update_user(
        State(state): State<AppState>,
        Json(user): Json<User>
    ) -> Result<String, AppError> {
        UserService::do_update_user(&state, user).await.map_err(|e| AppError::DatabaseError(e))?;
        Ok("update user success!".to_string())
    }
    

    
    pub async fn delete_user(
        State(state): State<AppState>,
        Json(user): Json<User>
    ) -> Result<String, AppError> {
        UserService::do_delete_user(&state, user).await.map_err(|e| AppError::DatabaseError(e))?;
        Ok("delete user success!".to_string())
    }
    

    
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