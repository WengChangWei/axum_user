pub struct UserService;

use sea_orm::*;

use crate::{
    boot_server::state::AppState, 
    entitys::{prelude::*, users_demo::Model, *}, vo::user::User, 
};

impl UserService {
    pub async fn do_create_user(
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

    pub async fn do_get_user(
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

    pub async fn do_update_user(
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

    pub async fn do_delete_user(
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