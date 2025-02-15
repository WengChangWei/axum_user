pub struct UserService;

use sea_orm::*;

use crate::{
    boot_server::state::AppState, 
    entitys::users_demo::Model, 
    vo::user::User, 
    dao::user_dao::UserDao,
};

impl UserService {
    pub async fn do_create_user(
        state: &AppState, 
        user: User
    ) -> Result<(), DbErr> {
        UserDao::create_model(&state.conn, user).await
    }

    pub async fn do_get_user(
        state: &AppState, 
        username: String
    ) -> Result<Model, DbErr> {
        UserDao::get_by_username(&state.conn, &username).await
    }

    pub async fn do_update_user(
        state: &AppState,
        user: User
    ) -> Result<(), DbErr> {
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
    ) -> Result<(), DbErr> {
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