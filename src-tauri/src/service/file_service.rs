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

use crate::dao::file_dao::{FileService, PageResult};
use crate::entity::file::{ActiveModel, Model};
use crate::{AppResponse, DIR_TYPE, RESPONSE_CODE_ERROR, RESPONSE_CODE_SUCCESS};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateBody {
    pub name: String,
    pub pid: String,
    pub wid: String,
    pub r#type: String,
    pub content: Option<String>,
    pub path: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateContentBody {
    pub id: String,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNameBody {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ListGeneralBody {
    pub wid: String,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ListByPidBody {
    pub wid: String,
    pub pid: String,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ListByPageBody {
    pub page_size: u64,
    pub page_num: u64,
    pub wid: String,
    pub pid: String,
    pub r#type: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GeneralBody {
    pub wid: String,
    pub id: String,
}

pub async fn get_workspace_files(
    db: &DatabaseConnection,
    general_body: &ListGeneralBody,
) -> AppResponse<Vec<Model>> {
    match &general_body.r#type {
        None => match FileService::list_files_by_wid(db, &general_body.wid).await {
            Ok(result) => AppResponse::success(result),
            Err(err) => AppResponse::error(vec![], &err.to_string()),
        },
        Some(t) => match FileService::list_files_by_wid_and_type(db, &general_body.wid, t).await {
            Ok(result) => AppResponse::success(result),
            Err(err) => AppResponse::error(vec![], &err.to_string()),
        },
    }
}

pub async fn get_workspace_files_by_pid(
    db: &DatabaseConnection,
    body: &ListByPidBody,
) -> AppResponse<Vec<Model>> {
    match &body.r#type {
        None => match FileService::list_files_by_wid_and_pid(db, &body.wid, &body.pid).await {
            Ok(result) => AppResponse::success(result),
            Err(err) => AppResponse::error(vec![], &err.to_string()),
        },
        Some(t) => {
            match FileService::list_files_by_wid_and_pid_and_type(db, &body.wid, &body.pid, t).await
            {
                Ok(result) => AppResponse::success(result),
                Err(err) => AppResponse::error(vec![], &err.to_string()),
            }
        }
    }
}

pub async fn get_workspace_files_by_page(
    db: &DatabaseConnection,
    body: &ListByPageBody,
) -> AppResponse<PageResult> {
    match FileService::list_files_by_page(
        db,
        body.page_size,
        body.page_num,
        &body.wid,
        &body.pid,
        &body.r#type,
        &body.name,
    )
    .await
    {
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
    user_path: &PathBuf,
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

#[cfg(test)]
mod test {
    use std::env::temp_dir;

    use sea_orm::{ConnectionTrait, Schema};
    use uuid::Uuid;

    use crate::service::file_service::{
        create_file, delete_file, get_file, get_workspace_files, CreateBody, GeneralBody,
        ListGeneralBody,
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
        // db.execute(builder.build(&schema.create_table_from_entity(entity::prelude::Workspace)))
        //     .await
        //     .unwrap();
        db.execute(builder.build(&schema.create_table_from_entity(entity::prelude::File)))
            .await
            .unwrap();
        // create workspace file,not do this
        let wid = Uuid::new_v4().to_string();
        // create dir
        let create_dir_response = create_file(
            db,
            user_path,
            &CreateBody {
                name: "test_dir".to_string(),
                pid: "".to_string(),
                wid: wid.clone(),
                r#type: DIR_TYPE.to_string(),
                content: None,
                path: None,
            },
        )
        .await;
        let dir_model = create_dir_response.result.unwrap();
        // get dir
        let get_dir_response = get_file(
            db,
            &GeneralBody {
                wid: wid.clone(),
                id: dir_model.id.clone(),
            },
        )
        .await;
        if !get_dir_response.is_success() {
            println!("create dir catch err: {}", get_dir_response.message);
            return;
        }
        // create file,parent is dir
        let create_file_response = create_file(
            db,
            user_path,
            &CreateBody {
                name: "test_file".to_string(),
                pid: dir_model.id.clone(),
                wid: wid.clone(),
                r#type: FILE_TYPE.to_string(),
                content: None,
                path: None,
            },
        )
        .await;
        let file_model = create_file_response.result.unwrap();
        // get file
        let get_file_response = get_file(
            db,
            &GeneralBody {
                wid: wid.clone(),
                id: file_model.id.clone(),
            },
        )
        .await;
        if !get_file_response.is_success() {
            println!("create dir catch err: {}", get_file_response.message);
            return;
        }
        // get all workspace files include file & dir
        let get_all_files_response = get_workspace_files(
            db,
            &ListGeneralBody {
                wid: wid.clone(),
                r#type: None,
            },
        )
        .await;
        if !get_all_files_response.is_success() {
            println!("create dir catch err: {}", get_all_files_response.message);
            return;
        }
        assert_eq!(2, get_all_files_response.result.len());
        // get all workspace files
        let get_files_response = get_workspace_files(
            db,
            &ListGeneralBody {
                wid: wid.clone(),
                r#type: Some(FILE_TYPE.to_string()),
            },
        )
        .await;
        assert_eq!(1, get_files_response.result.len());
        // get all workspace dirs
        let get_dirs_response = get_workspace_files(
            db,
            &ListGeneralBody {
                wid: wid.clone(),
                r#type: Some(DIR_TYPE.to_string()),
            },
        )
        .await;
        assert_eq!(1, get_dirs_response.result.len());
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
        let get_all_files_response_2 = get_workspace_files(
            db,
            &ListGeneralBody {
                wid: wid.clone(),
                r#type: None,
            },
        )
        .await;
        assert_eq!(0, get_all_files_response_2.result.len());
    }
}
