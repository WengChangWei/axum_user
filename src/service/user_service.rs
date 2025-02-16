pub struct UserService;

use crate::{
    boot_server::state::AppState, dao::user_dao::UserDao, entitys::users_demo::Model, error::error::AppError, vo::user::User
};

impl UserService {
    pub async fn do_create_user(
        state: &AppState, 
        user: User
    ) -> Result<(), AppError> {
        UserDao::create_model(&state.conn, user).await
    }

    pub async fn do_get_user(
        state: &AppState, 
        username: String
    ) -> Result<Model, AppError> {
        UserDao::get_by_username(&state.conn, &username).await
    }

    pub async fn do_update_user(
        state: &AppState,
        user: User
    ) -> Result<(), AppError> {
        let username = &user.username;
        let db = &state.conn;
        let user_model = UserDao::get_by_username(&state.conn, username).await;
        match user_model {
            Ok(model) => {
                UserDao::update_model(&db, model.id, user).await?;
                Ok(())
            },
            Err(e) => {
                Err(e)
            },
        }
    }

    pub async fn do_delete_user(
        state: &AppState,
        user: User
    ) -> Result<(), AppError> {
        let user_model = UserDao::get_by_username(&state.conn, &user.username).await;
        match user_model {
            Ok(model) => {
                UserDao::delete_by_username(&state.conn,model.id).await?;
                Ok(())
            },
            Err(e) => {
                Err(e)
            },
        }
    }
}