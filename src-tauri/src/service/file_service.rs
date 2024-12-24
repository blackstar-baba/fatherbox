use chrono::Utc;
use log::error;
use sea_orm::{DatabaseConnection, DbErr, Set};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tauri::api::dir::is_dir;
use uuid::Uuid;

use crate::dao::file_dao::FileService;
use crate::dto::file_dto::{
    CreateBody, GeneralBody, ListByPageBody, ListByPidBody, ListGeneralBody, PageResult,
    UpdateContentBody, UpdateNameBody,
};
use crate::entity::file::{ActiveModel, Model};
use crate::{AppResponse, DIR_TYPE, RESPONSE_CODE_ERROR, RESPONSE_CODE_SUCCESS};

pub async fn get_workspace_files(
    db: &DatabaseConnection,
    general_body: &ListGeneralBody,
) -> AppResponse<Vec<Model>> {
    match FileService::list_files(db, general_body).await {
        Ok(result) => AppResponse::success(result),
        Err(err) => AppResponse::error(vec![], &err.to_string()),
    }
}

pub async fn get_workspace_files_by_pid(
    db: &DatabaseConnection,
    body: &ListByPidBody,
) -> AppResponse<Vec<Model>> {
    match FileService::list_files_by_pid(db, body).await {
        Ok(result) => AppResponse::success(result),
        Err(err) => AppResponse::error(vec![], &err.to_string()),
    }
}

pub async fn get_workspace_files_by_page(
    db: &DatabaseConnection,
    body: &ListByPageBody,
) -> AppResponse<PageResult> {
    match FileService::list_files_by_page(db, body).await {
        Ok(result) => AppResponse::success(result),
        Err(err) => AppResponse::error(
            PageResult {
                total: 0,
                items: vec![],
            },
            &err.to_string(),
        ),
    }
}

pub async fn get_file(
    db: &DatabaseConnection,
    general_body: &GeneralBody,
) -> AppResponse<Option<Model>> {
    match FileService::get_file(db, &general_body.id).await {
        Ok(option_model) => AppResponse::success(option_model),
        Err(err) => AppResponse::error(None::<Model>, &err.to_string()),
    }
}

pub async fn get_path(user_path: &PathBuf, general_body: &GeneralBody) -> AppResponse<String> {
    let file_path = &user_path.join(&general_body.wid).join(&general_body.id);
    AppResponse::success(file_path.to_str().unwrap().to_string())
}

pub async fn create_file(
    db: &DatabaseConnection,
    user_path: &PathBuf,
    general_body: &CreateBody,
) -> AppResponse<Option<Model>> {
    // todo check parent first
    // if type is dir and parent is file, no allow to create
    let create_response = match FileService::create_file(
        db,
        ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            name: Set(general_body.name.clone()),
            r#type: Set(general_body.r#type.to_string()),
            pid: Set(general_body.pid.to_string()),
            wid: Set(general_body.wid.to_string()),
            zone: Set(general_body.zone.to_string()),
            size: Set(0),
            create_time: Set(Utc::now().timestamp()),
            update_time: Set(Utc::now().timestamp()),
            state: Set(1),
            ..Default::default()
        },
    )
    .await
    {
        Ok(model) => AppResponse::success(Some(model)),
        Err(err) => AppResponse::error(None::<Model>, &err.to_string()),
    };
    if create_response.is_error() {
        return create_response;
    }
    let file_model = create_response.result.unwrap();
    let file_path = &user_path.join(&general_body.wid).join(&file_model.id);
    if !file_path.exists() {
        if general_body.r#type == DIR_TYPE {
            fs::create_dir_all(file_path).unwrap();
        } else {
            let result = File::create(file_path);
            if result.is_err() {
                error!("create file on disk failed, err: {}", result.err().unwrap());
                return AppResponse::error(
                    None,
                    "create file on disk failed, please check your disk space and permissions",
                );
            }
        }
    }
    // direct return if type is dir
    if file_model.r#type == DIR_TYPE {
        return AppResponse::success(Some(file_model));
    }
    let mut size = 0i64;
    // insert content or copy file
    if general_body.content.is_some() {
        let open_result = File::options().write(true).open(file_path);
        if open_result.is_err() {
            error!(
                "open file on disk failed, err: {}",
                open_result.err().unwrap()
            );
            return AppResponse::error(
                None,
                "open file on disk failed, please check your disk space and permissions",
            );
        }
        let mut file = open_result.unwrap();
        let content = general_body.content.as_ref().unwrap();
        if content.len() > 0 {
            file.write_all(content.as_bytes()).unwrap();
        }
        size = file.metadata().unwrap().len() as i64;
    } else if general_body.path.is_some() {
        // extract this
        let path = general_body.path.as_ref().unwrap();
        fs::copy(path, file_path).unwrap();
        size = file_path.metadata().unwrap().len() as i64;
    }
    // update file model size
    match FileService::update_file_size(db, &file_model.id, size).await {
        Ok(_) => AppResponse::success(Some(file_model)),
        Err(err) => AppResponse::error(None, &err.to_string()),
    }
}

pub async fn update_file_content(
    db: &DatabaseConnection,
    user_path: &PathBuf,
    body: &UpdateContentBody,
) -> AppResponse<Option<Model>> {
    let get_result = FileService::get_file(db, &body.id).await;
    if get_result.is_err() {
        return AppResponse::error(None, &get_result.err().unwrap().to_string());
    }
    let option_model = get_result.unwrap();
    if option_model.is_none() {
        return AppResponse::error(None, "file not found");
    }
    let mut model = option_model.unwrap();
    if model.r#type == DIR_TYPE {
        return AppResponse::error(None, "dir can not update");
    }
    let file_path = &user_path.join(&model.wid).join(&model.id);
    let open_result = File::options().write(true).truncate(true).open(file_path);
    if open_result.is_err() {
        error!(
            "open file on disk failed, err: {}",
            open_result.err().unwrap()
        );
        return AppResponse::error(
            None,
            "open file on disk failed, please check your disk space and permissions",
        );
    }
    let mut file = open_result.unwrap();
    let content = &body.content;
    if content.len() > 0 {
        file.write_all(content.as_bytes()).unwrap();
    }
    let size = file.metadata().unwrap().len() as i64;
    model.size = size;
    match FileService::update_file_size(db, &body.id, size).await {
        Ok(_) => AppResponse::success(Some(model)),
        Err(err) => AppResponse::error(None, &err.to_string()),
    }
}

pub async fn update_file_name(
    db: &DatabaseConnection,
    body: &UpdateNameBody,
) -> AppResponse<Option<Model>> {
    let get_result = FileService::get_file(db, &body.id).await;
    if get_result.is_err() {
        return AppResponse::error(None, &get_result.err().unwrap().to_string());
    }
    let option_model = get_result.unwrap();
    if option_model.is_none() {
        return AppResponse::error(None, "file not found");
    }
    let model = option_model.unwrap();
    match FileService::update_file_name(db, &body.id, &body.name).await {
        Ok(_) => AppResponse::success(Some(model)),
        Err(err) => AppResponse::error(None, &err.to_string()),
    }
}

pub async fn delete_file(
    db: &DatabaseConnection,
    general_body: &GeneralBody,
) -> AppResponse<String> {
    // todo if exist files, do not allow to delete
    match FileService::delete_file(db, &general_body.id).await {
        Ok(_) => AppResponse::success("".to_string()),
        Err(err) => AppResponse::error("".to_string(), &err.to_string()),
    }
}

pub async fn list_workspace_zones(db: &DatabaseConnection, wid: &str) -> AppResponse<Vec<String>> {
    match FileService::list_workspace_zones(db, wid).await {
        Ok(result) => AppResponse::success(result),
        Err(err) => AppResponse::error(vec![], &err.to_string()),
    }
}

#[cfg(test)]
mod test {
    use std::env::temp_dir;

    use sea_orm::{ConnectionTrait, Schema};
    use uuid::Uuid;

    use crate::service::file_service::{
        create_file, delete_file, get_file, get_workspace_files, list_workspace_zones, CreateBody,
        GeneralBody, ListGeneralBody,
    };
    use crate::util::db_util::{drop_database_file, exist_database_file, init_connection};
    use crate::{entity, DIR_TYPE, FILE_TYPE};

    #[tokio::test]
    async fn test_files() {
        let workspace = "test";
        let temp_dir = temp_dir();
        let base_path = &temp_dir.join(".fatherbox");
        let user_path = base_path;
        let file_path = &base_path.join("test-file.sqlite");
        if exist_database_file(file_path) {
            drop_database_file(&file_path).unwrap();
        }
        let db = &init_connection(&file_path).await.unwrap();
        let builder = db.get_database_backend();
        let schema = Schema::new(builder);
        db.execute(builder.build(&schema.create_table_from_entity(entity::prelude::File)))
            .await
            .unwrap();
        // create workspace file,not do this
        let wid = Uuid::new_v4().to_string();
        let zone_1 = "zone-1";
        let zone_2 = "zone-2";
        // create dir
        let zone_1_dir_1 = "test_dir";
        let mut dir_model = create_file(
            db,
            user_path,
            &CreateBody {
                name: zone_1_dir_1.to_string(),
                pid: "".to_string(),
                wid: wid.clone(),
                r#type: DIR_TYPE.to_string(),
                zone: zone_1.to_string(),
                content: None,
                path: None,
            },
        )
        .await
        .result
        .unwrap();
        assert_eq!(zone_1_dir_1, dir_model.name);
        assert_eq!(zone_1, dir_model.zone);
        assert_eq!(wid, dir_model.wid);
        // get dir
        dir_model = get_file(
            db,
            &GeneralBody {
                wid: wid.clone(),
                id: dir_model.id.clone(),
            },
        )
        .await
        .result
        .unwrap();
        assert_eq!(zone_1_dir_1, dir_model.name);
        assert_eq!(zone_1, dir_model.zone);
        assert_eq!(wid, dir_model.wid);
        // create file, parent is dir
        let file_model = create_file(
            db,
            user_path,
            &CreateBody {
                name: "test_file".to_string(),
                pid: dir_model.id.clone(),
                wid: wid.clone(),
                r#type: FILE_TYPE.to_string(),
                zone: zone_1.to_string(),
                content: None,
                path: None,
            },
        )
        .await
        .result
        .unwrap();
        // get all workspace files include file & dir
        let zone_1_models = get_workspace_files(
            db,
            &ListGeneralBody {
                wid: wid.clone(),
                zone: zone_1.to_string(),
                r#type: None,
            },
        )
        .await
        .result;
        assert_eq!(2, zone_1_models.len());
        // list zone 1
        let mut zones = list_workspace_zones(db, &wid).await.result;
        assert_eq!(1, zones.len());
        assert_eq!(zone_1, zones[0]);
        let zone_2_models = get_workspace_files(
            db,
            &ListGeneralBody {
                wid: wid.clone(),
                zone: zone_2.to_string(),
                r#type: None,
            },
        )
        .await
        .result;
        assert_eq!(0, zone_2_models.len());
        // get all workspace files
        let zone_1_file_models = get_workspace_files(
            db,
            &ListGeneralBody {
                wid: wid.clone(),
                zone: zone_1.to_string(),
                r#type: Some(FILE_TYPE.to_string()),
            },
        )
        .await
        .result;
        assert_eq!(1, zone_1_file_models.len());
        let zone_2_file_models = get_workspace_files(
            db,
            &ListGeneralBody {
                wid: wid.clone(),
                zone: zone_2.to_string(),
                r#type: Some(FILE_TYPE.to_string()),
            },
        )
        .await
        .result;
        assert_eq!(0, zone_2_file_models.len());
        // get all workspace dirs
        let zone_1_dir_models = get_workspace_files(
            db,
            &ListGeneralBody {
                wid: wid.clone(),
                zone: zone_1.to_string(),
                r#type: Some(DIR_TYPE.to_string()),
            },
        )
        .await
        .result;
        assert_eq!(1, zone_1_dir_models.len());
        // delete dir, must failed, because dir is parent

        // delete file
        let delete_file_response = delete_file(
            db,
            &GeneralBody {
                wid: wid.clone(),
                id: file_model.id.clone(),
            },
        )
        .await;
        assert_eq!(true, delete_file_response.is_success());
        // delete dir
        let delete_dir_response = delete_file(
            db,
            &GeneralBody {
                wid: wid.clone(),
                id: dir_model.id.clone(),
            },
        )
        .await;
        assert_eq!(true, delete_dir_response.is_success());
        // get all files include dir
        let zone_1_after_delete_models = get_workspace_files(
            db,
            &ListGeneralBody {
                wid: wid.clone(),
                zone: zone_1.to_string(),
                r#type: None,
            },
        )
        .await
        .result;
        assert_eq!(0, zone_1_after_delete_models.len());
    }
}
