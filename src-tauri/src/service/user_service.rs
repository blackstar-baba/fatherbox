use futures::{StreamExt, TryFutureExt};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait,
    FromQueryResult, ModelTrait, PaginatorTrait, QueryFilter, Statement,
};

use crate::entity::user;
use crate::entity::user::{
    ActiveModel as UserActiveModel, DataTransModel as UserDataTransModel, DataTransModel,
    Entity as User, Model as UserModel,
};

pub struct UserService;

impl UserService {
    pub async fn create_user(
        db: &DatabaseConnection,
        user: UserActiveModel,
    ) -> Result<UserModel, DbErr> {
        user.insert(db).await.and_then(|mut user| {
            user.password = "".to_owned();
            return Ok(user);
        })
    }

    pub async fn exist_user(
        db: &DatabaseConnection,
        username: &str,
        r#type: &str,
    ) -> Result<bool, DbErr> {
        User::find()
            .filter(user::Column::Username.eq(username))
            .filter(user::Column::Type.eq(r#type))
            .count(db)
            .await
            .and_then(|num| {
                if num == 0 {
                    return Ok(false);
                }
                return Ok(true);
            })
    }

    pub async fn get_user_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Vec<DataTransModel>, DbErr> {
        let query = Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT u1.id, u1.username, u1.nickname, u1.avatar, u1.mail, u1.type, u1.ref_user_id,
            u1.create_time, u1.update_time, u1.state, u2.username as ref_user_name
            FROM user u1
            LEFT JOIN user u2 on u1.ref_user_id = u2.id
            WHERE u1.id = ?",
            [id.into()],
        );
        UserDataTransModel::find_by_statement(query).all(db).await
    }

    pub async fn get_user(
        db: &DatabaseConnection,
        username: &str,
        password: &str,
        r#type: &str,
    ) -> Result<Vec<UserDataTransModel>, DbErr> {
        let query = Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT u1.id, u1.username, u1.nickname, u1.avatar, u1.mail, u1.type, u1.ref_user_id,
            u1.create_time, u1.update_time, u1.state, u2.username as ref_user_name
            FROM user u1
            LEFT JOIN user u2 on u1.ref_user_id = u2.id
            WHERE u1.username = ? AND u1.password = ? AND u1.type = ?",
            [username.into(), password.into(), r#type.into()],
        );
        // todo use one to replace all
        UserDataTransModel::find_by_statement(query).all(db).await
    }
}
