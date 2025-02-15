use sea_orm::{DatabaseConnection, DbErr};

use sea_orm::*;
use crate::{
    entitys::{prelude::*, users_demo::Model, *}, 
    vo::user::User,
};

pub struct UserDao;

impl UserDao {
    pub async fn create_model(
        db: &DatabaseConnection,
        user: User
    ) -> Result<(), DbErr> {
        let user_model: users_demo::ActiveModel = users_demo::ActiveModel {
            id: ActiveValue::NotSet,
            username: ActiveValue::Set(user.username),
            email: ActiveValue::Set(user.email),
            ..Default::default()
        };
        let res = UsersDemo::insert(user_model).exec(db).await;
        match res {
            Ok(_) => {
                Ok(())
            },
            Err(e) => {
                Err(e)
            },
        }
    }

    pub async fn get_by_username(
        db: &DatabaseConnection,
        username: &String
    ) -> Result<Model, DbErr> {
        let user = UsersDemo::find()
            .filter(users_demo::Column::Username.eq(username))
            .one(db)
            .await?;
        match user {
            Some(s) => {
                return Ok(s)
            },
            None => {
                Err(DbErr::RecordNotFound(format!("username: {} is not found", &username)))
            },
        }
    }

    pub async fn update_model(
        db: &DatabaseConnection,
        user_id: i32,
        user: User
    ) -> Result<(), DbErr> {
        let db_model = users_demo::ActiveModel {
            id: ActiveValue::Set(user_id),
            username: ActiveValue::Set(user.username),
            email: ActiveValue::Set(user.email),
            ..Default::default()
        };
        let res = db_model.update(db).await;
        match res {
            Ok(_) => {
                Ok(())
            },
            Err(e) => {
                Err(e)
            },
        }
    }

    pub async fn delete_by_username(
        db: &DatabaseConnection,
        user_id: i32
    ) -> Result<(), DbErr> {
        let user_model = users_demo::ActiveModel {
            id: ActiveValue::Set(user_id),
            ..Default::default()
        };
        let res = user_model.delete(db).await;
        match res {
            Ok(_) => {
                Ok(())
            },
            Err(e) => {
                Err(e)
            },
        }
    }
}