use sea_orm::DatabaseConnection;

use sea_orm::*;
use snafu::{OptionExt, ResultExt};
use crate::{
    entitys::{
        prelude::*, 
        users_demo::Model as UsersDemoModel, 
        *
    }, 
    vo::user::User,
    error::api_return::{DataNotFoundSnafu, MetaDataBaseAccessErrSnafu, Result}
};

pub struct UserDao;

impl UserDao {
    pub async fn create_model(
        db: &DatabaseConnection,
        user: User
    ) -> Result<()> {
        let user_model: users_demo::ActiveModel = users_demo::ActiveModel {
            id: ActiveValue::NotSet,
            username: ActiveValue::Set(user.username),
            email: ActiveValue::Set(user.email),
            ..Default::default()
        };
        UsersDemo::insert(user_model)
        .exec(db)
        .await
        .context(MetaDataBaseAccessErrSnafu)?;
        Ok(())
    }

    pub async fn get_by_username(
        db: &DatabaseConnection,
        username: &String
    ) -> Result<UsersDemoModel> {
        UsersDemo::find()
            .filter(users_demo::Column::Username.eq(username))
            .one(db)
            .await
            .context(MetaDataBaseAccessErrSnafu)?
            .context(DataNotFoundSnafu {
                message: format!("Could not find username: {username}"),
            })
    }

    pub async fn update_model(
        db: &DatabaseConnection,
        user_id: i32,
        user: User
    ) -> Result<()> {
        let db_model = users_demo::ActiveModel {
            id: ActiveValue::Set(user_id),
            username: ActiveValue::Set(user.username),
            email: ActiveValue::Set(user.email),
            ..Default::default()
        };
        db_model.update(db)
        .await        
        .context(MetaDataBaseAccessErrSnafu)?;
        Ok(())
    }

    pub async fn delete_by_username(
        db: &DatabaseConnection,
        user_id: i32
    ) -> Result<()> {
        let user_model = users_demo::ActiveModel {
            id: ActiveValue::Set(user_id),
            ..Default::default()
        };
        user_model.delete(db)
        .await        
        .context(MetaDataBaseAccessErrSnafu)?;
        Ok(())
    }
}