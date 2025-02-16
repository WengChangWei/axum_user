use axum::{extract::{Path, State}, Json};
use crate::{
    api_ok, boot_server::state::AppState, entitys::users_demo::Model as UsersDemoModel, error::{ api_result::ApiOk, error::AppError}, service::user_service::UserService, vo::user::User
};

pub struct UserHandler;

impl UserHandler {
    
    pub async fn create_user(
        State(state): State<AppState>,
        Json(user): Json<User>
    ) -> Result<ApiOk<String>, AppError> {
        UserService::do_create_user(&state,user)
        .await
        .map(|_r| api_ok!("success".to_string()))?
    }

    
    pub async fn get_user(
        State(state): State<AppState>,
        Path(name): Path<String>
    ) -> Result<ApiOk<UsersDemoModel>, AppError> {
        UserService::do_get_user(&state, name)
        .await
        .map(|r| api_ok!(r))?
    }
    
    
    pub async fn update_user(
        State(state): State<AppState>,
        Json(user): Json<User>
    ) -> Result<ApiOk<String>, AppError> {
        UserService::do_update_user(&state, user)
        .await
        .map(|_r| api_ok!("success".to_string()))?
    }
    

    
    pub async fn delete_user(
        State(state): State<AppState>,
        Json(user): Json<User>
    ) -> Result<ApiOk<String>, AppError> {
        UserService::do_delete_user(&state, user)
        .await
        .map(|_r| api_ok!("success".to_string()))?
    }
    

    
}