use crate::entity::ai_source;
use crate::entity::ai_source::{ActiveModel, Entity, Model};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter};

pub struct AiConnectionService;

impl AiConnectionService {
    pub async fn create(
        db: &DatabaseConnection,
        active_model: ActiveModel,
    ) -> Result<Model, DbErr> {
        active_model.insert(db).await
    }

    pub async fn get(db: &DatabaseConnection, id: &str) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id.to_string()).one(db).await
    }

    pub async fn get_by_name(
        db: &DatabaseConnection,
        name: &str,
    ) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(ai_source::Column::Name.eq(name))
            .one(db)
            .await
    }

    pub async fn update(
        db: &DatabaseConnection,
        active_model: ActiveModel,
    ) -> Result<Model, DbErr> {
        active_model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), DbErr> {
        let action_model = ActiveModel {
            id: Set(id.to_string()),
            ..Default::default()
        };
        match action_model.delete(db).await {
            Ok(_) => {
                Ok(())
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub async fn list(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        Entity::find().all(db).await
    }

    pub async fn list_enable(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(ai_source::Column::Enable.eq(true))
            .all(db).await
    }
}
