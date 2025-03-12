use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::dao::ai_model_dao::AiModelService;
use crate::dto::ai_model::{CreateBody, EnableBody, UpdateBody};
use crate::entity::ai_model::{ActiveModel, Model};
use crate::AppResponse;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GeneralBody {
    pub key: String,
}

pub async fn create(db: &DatabaseConnection, body: &CreateBody) -> AppResponse<Option<Model>> {
    let active_model = ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        name: Set(body.name.clone()),
        source_id: Set(body.source_id.clone()),
        enable: Set(true),
        create_time: Set(Utc::now().timestamp()),
        update_time: Set(Utc::now().timestamp()),
        state: Set(1),
        ..Default::default()
    };
    match AiModelService::create(db, active_model).await {
        Ok(model) => AppResponse::success(Some(model)),
        Err(err) => AppResponse::error(None, &err.to_string()),
    }
}

pub async fn list(db: &DatabaseConnection, source_id: &str) -> AppResponse<Vec<Model>> {
    match AiModelService::list(db, source_id).await {
        Ok(model) => AppResponse::success(model),
        Err(err) => AppResponse::error(vec![], &err.to_string()),
    }
}

pub async fn get(db: &DatabaseConnection, id: &str) -> AppResponse<Option<Model>> {
    match AiModelService::get(db, id).await {
        Ok(model) => AppResponse::success(model),
        Err(err) => AppResponse::error(None, &err.to_string()),
    }
}

pub async fn update(db: &DatabaseConnection, body: &UpdateBody) -> AppResponse<Option<Model>> {
    let active_model = ActiveModel {
        id: Set(body.id.clone()),
        name: Set(body.name.clone()),
        ..Default::default()
    };
    match AiModelService::update(db, active_model).await {
        Ok(model) => AppResponse::success(Some(model)),
        Err(err) => AppResponse::error(None, &err.to_string()),
    }
}

pub async fn delete(db: &DatabaseConnection, id: &str) -> AppResponse<Option<Model>> {
    match AiModelService::delete(db, id).await {
        Ok(()) => AppResponse::success(None),
        Err(err) => AppResponse::error(None, &err.to_string()),
    }
}

pub async fn enable(db: &DatabaseConnection, body: &EnableBody) -> AppResponse<Option<Model>> {
    // is build-in
    let active_model = ActiveModel {
        id: Set(body.id.clone()),
        enable: Set(body.enable.clone()),
        ..Default::default()
    };
    match AiModelService::update(db, active_model).await {
        Ok(model) => AppResponse::success(Some(model)),
        Err(err) => AppResponse::error(None, &err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::dto::ai_model::{CreateBody, UpdateBody};
    use crate::dto::ai_source::CreateBody as ConnectionCreateBody;
    use crate::service::ai_model_service::{create, delete, get, update};
    use crate::service::ai_source_service::{create as create_connection, delete as delete_connection};
    use crate::util::db_util::init_test_database;

    #[tokio::test]
    async fn test_ai_connection() {
        let db = &init_test_database(
            "test-ai-model",
            &vec!["ai_connection".to_string(), "ai_model".to_string()],
        )
        .await
        .unwrap();

        let c_name = "connection1";
        let c_key = "sn-aaa";
        let c_url = "https://xxxx.aaa.com";
        // begin invoke
        // 0. create connection
        let connection = create_connection(
            db,
            &ConnectionCreateBody {
                name: c_name.to_string(),
                key: c_key.to_string(),
                url: c_url.to_string(),
            },
        )
        .await
        .result
        .unwrap();
        let c_id = connection.id.as_str();
        // 1. test create
        let name = "model1";
        let result = create(
            db,
            &CreateBody {
                name: name.to_string(),
                source_id: c_id.to_string(),
            },
        )
        .await;
        if result.is_error() {
            panic!("{:?}", result.message);
        }
        let model = result.result.unwrap();
        let id = &model.id;
        assert_eq!(name, model.name);
        assert_eq!(c_id, model.source_id);
        // 2. test get
        let result = get(db, id).await;
        if result.is_error() {
            panic!("{:?}", result.message);
        }
        let model = result.result.unwrap();
        assert_eq!(name, model.name);
        // 3. test update
        let new_name = "model2";

        let result = update(
            db,
            &UpdateBody {
                id: id.to_string(),
                name: new_name.to_string(),
            },
        )
        .await;
        if result.is_error() {
            panic!("{:?}", result.message);
        }
        let model = result.result.unwrap();
        assert_eq!(new_name, model.name);
        // 4. test delete
        let result = delete(db, id).await;
        if result.is_error() {
            panic!("{:?}", result.message);
        }
        let result = get(db, id).await;
        if result.is_error() {
            panic!("{:?}", result.message);
        }
        assert!(result.result.is_none());
        // delete connection
        delete_connection(db, c_id).await;
    }
}
