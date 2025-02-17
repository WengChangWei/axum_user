use axum::{extract::{Path, State}, Json};
use crate::{
    boot_server::state::AppState, 
    entitys::users_demo::Model as UsersDemoModel, 
    error::{ api_return::AppSuccess, api_return::AppError}, 
    service::user_service::UserService, 
    success, 
    vo::user::User
};

pub struct UserHandler;

impl UserHandler {
    
    pub async fn create_user(
        State(state): State<AppState>,
        Json(user): Json<User>
    ) -> Result<AppSuccess<()>, AppError> {
        UserService::do_create_user(&state,user)
        .await
        .map(|r| success!(r))?
    }

    
    pub async fn get_user(
        State(state): State<AppState>,
        Path(name): Path<String>
    ) -> Result<AppSuccess<UsersDemoModel>, AppError> {
        UserService::do_get_user(&state, name)
        .await
        .map(|r| success!(r))?
    }
    
    
    pub async fn update_user(
        State(state): State<AppState>,
        Json(user): Json<User>
    ) -> Result<AppSuccess<()>, AppError> {
        UserService::do_update_user(&state, user)
        .await
        .map(|r| success!(r))?
    }
    

    
    pub async fn delete_user(
        State(state): State<AppState>,
        Json(user): Json<User>
    ) -> Result<AppSuccess<()>, AppError> {
        UserService::do_delete_user(&state, user)
        .await
        .map(|r| success!(r))?
    }
    

    
}