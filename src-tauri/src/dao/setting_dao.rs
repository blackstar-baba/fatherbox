use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    ModelTrait, QueryFilter,
};

use crate::entity::prelude::Setting;
use crate::entity::setting;
use crate::entity::setting::{ActiveModel, Model};

pub struct SettingService;

impl SettingService {
    pub async fn create_setting(
        db: &DatabaseConnection,
        setting: ActiveModel,
    ) -> Result<Model, DbErr> {
        setting.insert(db).await
    }

    pub async fn delete_setting(db: &DatabaseConnection, key: &str) -> Result<(), DbErr> {
        if let Some(setting) = Setting::find_by_id(key.to_string()).one(db).await? {
            setting.delete(db).await?;
        }
        Ok(())
    }

    pub async fn update_setting(
        db: &DatabaseConnection,
        setting: ActiveModel,
    ) -> Result<Option<Model>, DbErr> {
        if let Some(existing_setting) = Setting::find_by_id(setting.key.clone().unwrap())
            .one(db)
            .await?
        {
            let mut active_model = existing_setting.into_active_model();

            active_model.value = setting.value.clone();

            let updated_setting = active_model.update(db).await?;
            Ok(Some(updated_setting))
        } else {
            Ok(None)
        }
    }

    pub async fn get_setting_by_key(
        db: &DatabaseConnection,
        key: &str,
    ) -> Result<Option<Model>, DbErr> {
        Setting::find()
            .filter(setting::Column::Key.eq(key))
            .one(db)
            .await
    }
}
