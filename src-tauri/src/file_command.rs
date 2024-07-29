use std::fs;
use std::fs::File;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::Utc;
use sea_orm::{DatabaseConnection, Set};
use tauri::State;
use uuid::Uuid;

use app::entity::file::{ActiveModel, DataTransModel, Model};
use app::service::file_service::FileService;
use app::service::workspace_service::WorkspaceService;
use app::{
    AppResponse, AppState, FileEntry, DIR_TYPE, FILE_TYPE, RESPONSE_CODE_ERROR,
    RESPONSE_CODE_SUCCESS, WORKSPACE_PATH,
};

#[tauri::command]
pub async fn list_workspace_files_cmd(
    state: State<'_, AppState>,
    wid: String,
    pid: String,
) -> Result<AppResponse<Vec<Model>>, ()> {
    list_workspace_files_inner(&state.conn, wid, pid).await
}

#[tauri::command]
pub async fn list_workspace_dirs_cmd(
    state: State<'_, AppState>,
    wid: String,
) -> Result<AppResponse<Vec<Model>>, ()> {
    list_workspace_dirs_inner(&state.conn, wid).await
}

#[tauri::command]
pub async fn create_workspace_file_cmd(
    state: State<'_, AppState>,
    wid: String,
    pid: String,
    file_type: String,
    file_name: String,
) -> Result<AppResponse<Option<Model>>, ()> {
    create_workspace_file_inner(
        &state.conn,
        &state.workspace_path,
        wid,
        pid,
        file_type,
        file_name,
    )
    .await
}

#[tauri::command]
pub async fn update_workspace_file_cmd(
    state: State<'_, AppState>,
    id: String,
    wid: String,
    pid: String,
    file_type: String,
    file_name: String,
) -> Result<AppResponse<Option<Model>>, ()> {
    update_workspace_file_inner(
        &state.conn,
        id,
        wid,
        pid,
        file_type,
        file_name,
    ).await
}

#[tauri::command]
pub async fn get_workspace_file(
    state: State<'_, AppState>,
    id: String,
) -> Result<AppResponse<Option<DataTransModel>>, ()> {
    get_workspace_file_inner(&state.conn, id).await
}

#[tauri::command]
pub async fn delete_workspace_file_cmd(
    state: State<'_, AppState>,
    id: String,
) -> Result<AppResponse<String>, ()> {
    delete_workspace_file_inner(&state.conn, &state.workspace_path, id).await
}

async fn list_workspace_files_inner(
    db: &DatabaseConnection,
    wid: String,
    pid: String,
) -> Result<AppResponse<Vec<Model>>, ()> {
    let vec = FileService::list_files_by_workspace_and_type_and_parent(db, &wid, FILE_TYPE, &pid)
        .await
        .unwrap();
    Ok(AppResponse {
        code: RESPONSE_CODE_SUCCESS,
        r#type: "".to_owned(),
        message: "".to_owned(),
        result: vec,
    })
}

async fn list_workspace_dirs_inner(
    db: &DatabaseConnection,
    wid: String,
) -> Result<AppResponse<Vec<Model>>, ()> {
    let vec = FileService::list_files_by_workspace_and_type(db, &wid, DIR_TYPE)
        .await
        .unwrap();
    Ok(AppResponse {
        code: RESPONSE_CODE_SUCCESS,
        r#type: "".to_owned(),
        message: "".to_owned(),
        result: vec,
    })
}

pub async fn create_workspace_file_inner(
    db: &DatabaseConnection,
    workspace_path: &PathBuf,
    wid: String,
    pid: String,
    file_type: String,
    file_name: String,
) -> Result<AppResponse<Option<Model>>, ()> {
    let option_workspace_model = WorkspaceService::get_workspace(db, &wid).await.unwrap();
    if option_workspace_model.is_none() {
        return Ok(AppResponse {
            code: RESPONSE_CODE_ERROR,
            r#type: "".to_owned(),
            message: "".to_owned(),
            result: None,
        });
    };
    let workspace_model = option_workspace_model.unwrap();
    let mut file_path = workspace_path.join(workspace_model.name);
    // check parent
    if &pid != "" {
        let option_parent_file_model = FileService::get_file(db, &pid).await.unwrap();
        if option_parent_file_model.is_none() {
            return Ok(AppResponse {
                code: RESPONSE_CODE_ERROR,
                r#type: "".to_owned(),
                message: "".to_owned(),
                result: None,
            });
        };
    }

    let id = Uuid::new_v4().to_string();
    // create file
    file_path = file_path.join(&id);
    if file_type != DIR_TYPE {
        if !file_path.exists() {
            File::create(file_path).unwrap();
        }
    }
    // create file data
    let model = FileService::create_file(
        db,
        ActiveModel {
            id: Set(id),
            name: Set(file_name),
            r#type: Set(file_type),
            pid: Set(pid),
            wid: Set(wid),
            size: Set(0),
            create_time: Set(Utc::now().timestamp()),
            update_time: Set(Utc::now().timestamp()),
            state: Set(1),
        },
    )
    .await
    .unwrap();

    Ok(AppResponse {
        code: RESPONSE_CODE_SUCCESS,
        r#type: "".to_owned(),
        message: "".to_owned(),
        result: Some(model),
    })
}

pub async fn update_workspace_file_inner(
    db: &DatabaseConnection,
    id: String,
    wid: String,
    pid:String,
    file_type: String,
    file_name: String,
) -> Result<AppResponse<Option<Model>>, ()> {
    let option_file_model = FileService::get_file(db, &id).await.unwrap();
    if option_file_model.is_none() {
        return Ok(AppResponse {
            code: RESPONSE_CODE_ERROR,
            r#type: "".to_owned(),
            message: "".to_owned(),
            result: None,
        });
    };
    let file_model = option_file_model.unwrap();
    // check parent
    let file_active_model = ActiveModel {
        id: Set(id),
        name: Set(file_name),
        r#type: Set(file_type),
        pid: Set(pid),
        wid: Set(wid),
        size: Set(0),
        create_time: Set(file_model.create_time),
        update_time: Set(Utc::now().timestamp()),
        state: Set(file_model.state),
    };
    // create file
    let option_update_file_model = FileService::update_file(&db, file_active_model).await.unwrap();
    if option_update_file_model.is_none() {
        return Ok(AppResponse {
            code: RESPONSE_CODE_ERROR,
            r#type: "".to_owned(),
            message: "".to_owned(),
            result: None,
        });
    };

    Ok(AppResponse {
        code: RESPONSE_CODE_SUCCESS,
        r#type: "".to_owned(),
        message: "".to_owned(),
        result: Some(option_update_file_model.unwrap()),
    })
}

pub async fn delete_workspace_file_inner(
    db: &DatabaseConnection,
    workspace_path: &PathBuf,
    id: String,
) -> Result<AppResponse<String>, ()> {
    // get current file
    let option_file_model = FileService::get_file(db, &id).await.unwrap();
    if option_file_model.is_none() {
        return Ok(AppResponse {
            code: RESPONSE_CODE_ERROR,
            r#type: "".to_owned(),
            message: "".to_owned(),
            result: "".to_owned(),
        });
    };
    let file_model = option_file_model.unwrap();
    // todo if file is dir & has children, can not delete
    // delete file in file system
    let mut file_path = workspace_path
        .join(&file_model.workspace_name);
    if file_model.parent_file_name.is_some() {
        file_path = file_path.join(&file_model.parent_file_name.unwrap())
    }
    file_path = file_path.join(&file_model.id);
    if file_model.r#type != DIR_TYPE {
      fs::remove_file(file_path).unwrap();
    }
    // delete file in db
    FileService::delete_file(db, &id).await.unwrap();
    Ok(AppResponse {
        code: RESPONSE_CODE_SUCCESS,
        r#type: "".to_string(),
        message: "".to_string(),
        result: "".to_string(),
    })
}
pub async fn get_workspace_file_inner(
    db: &DatabaseConnection,
    id: String,
) -> Result<AppResponse<Option<DataTransModel>>, ()> {
    let option_file_model = FileService::get_file(db, &id).await.unwrap();
    return Ok(AppResponse {
        code: RESPONSE_CODE_ERROR,
        r#type: "".to_owned(),
        message: "".to_owned(),
        result: option_file_model,
    });
}

fn get_file_entry(
    base_path: &str,
    path: &str,
    recursive: bool,
    option_type_filter: Option<String>,
    option_name_filter: Option<String>,
) -> Vec<FileEntry> {
    let mut file_entrys = vec![];
    let full_path = Path::new(base_path).join(path);
    if let Ok(entries) = fs::read_dir(full_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let mut file_type = "";
                let mut create_epoch_ms = 0;
                let mut modified_epoch_ms = 0;
                let mut size = 0;
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        file_type = FILE_TYPE;
                    } else if metadata.is_dir() {
                        file_type = DIR_TYPE
                    } else {
                        // ignore unknown file
                        break;
                    }
                    create_epoch_ms = metadata
                        .created()
                        .unwrap()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis();

                    modified_epoch_ms = metadata
                        .modified()
                        .unwrap()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis();

                    size = metadata.size();
                }
                // let entry_path = entry.path();
                // if file_type == "file" {
                //     file_type = entry_path.extension().unwrap().to_str().unwrap()
                // }
                let abs_path = Path::new(path).join(entry.file_name());

                let mut children = vec![];
                if recursive {
                    children = get_file_entry(
                        base_path,
                        abs_path.to_str().unwrap(),
                        recursive.clone(),
                        option_type_filter.clone(),
                        option_name_filter.clone(),
                    )
                }

                let file_name = entry.file_name().clone().to_string_lossy().to_string();
                let file_entry = FileEntry {
                    name: file_name.clone(),
                    path: abs_path.to_string_lossy().to_string(),
                    parent_path: path.to_string(),
                    r#type: file_type.to_string(),
                    size,
                    create_time: create_epoch_ms,
                    modify_time: modified_epoch_ms,
                    children,
                };
                if (option_name_filter.is_none()) {
                    file_entrys.push(file_entry)
                } else {
                    let name_filter = option_name_filter.clone().unwrap();
                    if file_name.find(name_filter.as_str()).is_some() {
                        file_entrys.push(file_entry)
                    }
                }
            }
        }
        // filter type
        if option_type_filter.is_some() {
            let type_filter = option_type_filter.unwrap();
            if file_entrys.len() != 0 {
                let mut filtered_file_entrys = vec![];
                for entry in file_entrys {
                    if entry.r#type == type_filter {
                        filtered_file_entrys.push(entry.clone());
                    }
                }
                file_entrys = filtered_file_entrys;
            }
        }
    }
    return file_entrys;
}

#[cfg(test)]
mod test {
    use std::env::temp_dir;
    use std::fs::File;
    use sea_orm::{ConnectionTrait, Schema};

    use app::{entity, DIR_TYPE, FILE_TYPE, WORKSPACE_PATH};

    use crate::db_utils::{drop_database_file, exist_database_file, init_connection};
    use crate::file_command::{create_workspace_file_inner, delete_workspace_file_inner, get_workspace_file, get_workspace_file_inner, list_workspace_dirs_inner, list_workspace_files_inner, update_workspace_file_inner};
    use crate::workspace_command::create_workspace_inner;

    #[tokio::test] //由此判断这是一个测试函数
    async fn test_files() {
        let workspace = "test";
        let temp_dir = temp_dir();
        let base_path = temp_dir.join(".fatherbox");
        let workspace_path = base_path.join(WORKSPACE_PATH);
        let file_path = &base_path.join("test-file.sqlite");
        if exist_database_file(file_path) {
            drop_database_file(&file_path).unwrap();
        }
        let db = init_connection(&file_path).await.unwrap();
        let builder = db.get_database_backend();
        let schema = Schema::new(builder);
        db.execute(builder.build(&schema.create_table_from_entity(entity::prelude::Workspace)))
            .await
            .unwrap();
        db.execute(builder.build(&schema.create_table_from_entity(entity::prelude::File)))
            .await
            .unwrap();
        // create workspace
        let workspace_model = create_workspace_inner(&db, workspace.to_owned())
            .await
            .unwrap()
            .result;
        // create dir
        let test_workspace_path = &workspace_path.join(workspace);
        if !test_workspace_path.exists() {
            File::create(test_workspace_path).unwrap();
        }
        // create file
        let option_simple_file_model = create_workspace_file_inner(
            &db,
            &workspace_path,
            workspace_model.id.clone(),
            "".to_string(),
            FILE_TYPE.to_owned(),
            "my_file.txt".to_owned(),
        )
        .await
        .unwrap()
        .result;
        assert_eq!(true, option_simple_file_model.is_some());
        assert_eq!("my_file.txt", option_simple_file_model.unwrap().name);
        // create dir
        let option_dir_file_model = create_workspace_file_inner(
            &db,
            &base_path,
            workspace_model.id.clone(),
            "".to_string(),
            DIR_TYPE.to_owned(),
            "my_dir".to_owned(),
        )
        .await
        .unwrap()
        .result;
        assert_eq!(true, option_dir_file_model.is_some());
        let dir_file_model = option_dir_file_model.unwrap();
        assert_eq!("my_dir", dir_file_model.name.clone());
        // list file
        let list_file_result =
            list_workspace_files_inner(&db, workspace_model.id.clone(), "".to_string())
                .await
                .unwrap()
                .result;
        assert_eq!(1, list_file_result.len());
        let list_dir_result = list_workspace_dirs_inner(&db, workspace_model.id.clone())
            .await
            .unwrap()
            .result;
        assert_eq!(1, list_dir_result.len());
        // get file
        let file_result = get_workspace_file_inner(&db, list_file_result[0].id.clone())
            .await
            .unwrap()
            .result;
        assert_eq!(true, file_result.is_some());
        assert_eq!(list_file_result[0].id.clone(), file_result.unwrap().id.clone());

        // update file
        let name = "new_my_dir";
        let update_result = update_workspace_file_inner(&db, dir_file_model.id.clone(), dir_file_model.wid.clone(), dir_file_model.pid.clone(), dir_file_model.r#type.clone(), name.to_owned()).await.unwrap().result;
        assert_eq!(true, update_result.is_some());
        assert_eq!(name, update_result.unwrap().name);
        // delete
        let _ = delete_workspace_file_inner(&db, &workspace_path, list_file_result[0].id.clone())
            .await
            .unwrap()
            .result;
        let file_result = get_workspace_file_inner(&db, list_file_result[0].id.clone())
            .await
            .unwrap()
            .result;
        assert_eq!(true, file_result.is_none());
    }
}