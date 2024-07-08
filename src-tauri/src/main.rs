// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::process::exit;
use std::time::SystemTime;
use std::{env, fs};

use clap::Parser;
use config::FileFormat;
use log::info;
use tauri::api::http::{ClientBuilder, HttpRequestBuilder, ResponseType};
use tauri::api::path::home_dir;

use app::api::Api;
use app::{
    AppResponse, Config, FileEntry, FileRequest, ModelData, CONFIG_PATH, DEFAULT_WORKSPACE,
    DIR_TYPE, FILE_TYPE, RESPONSE_CODE_ERROR, RESPONSE_CODE_SUCCESS, ROOT_PATH, WORKSPACE_PATH,
};

static DEFAULT_CONFIG: &str = include_str!("../config.toml");

#[derive(Parser)]
#[command(version)]
#[command(name = "fb")]
#[command(about = "Father Box .")]
#[command(author = "blackstar-baba <535650957@qq.com>")]
struct Args {
    /// path to config file
    #[arg(short, long)]
    config: Option<String>,
    /// log level (v: info, vv: debug, vvv: trace)
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    verbose: u8,
}

fn banner() {
    const B: &str = r"
        Father Box
    ";
    println!("{B}\n");
}

#[tauri::command]
fn my_custom_command() -> String {
    println!("I was invoked from JS!");
    return String::from("Hello, world!");
}

#[tauri::command]
fn create_workspace(workspace: String) -> AppResponse<String> {
    let home = home_dir().unwrap();
    let base_path = Path::join(home.as_path(), ROOT_PATH)
        .join(WORKSPACE_PATH)
        .join(workspace);
    if !base_path.exists() {
        fs::create_dir_all(base_path.clone()).unwrap();
    }
    AppResponse {
        code: RESPONSE_CODE_SUCCESS,
        r#type: "".to_string(),
        message: "".to_string(),
        result: base_path.to_string_lossy().to_string(),
    }
}

#[tauri::command]
fn list_workspaces() -> AppResponse<Vec<String>> {
    let home = home_dir().unwrap();
    let base_path = Path::join(home.as_path(), ROOT_PATH).join(WORKSPACE_PATH);
    let mut result = vec![];
    if let Ok(entries) = fs::read_dir(base_path) {
        for entry in entries {
            result.push(entry.unwrap().file_name().to_string_lossy().to_string())
        }
    }
    AppResponse {
        code: RESPONSE_CODE_SUCCESS,
        r#type: "".to_string(),
        message: "".to_string(),
        result,
    }
}

#[tauri::command]
fn delete_workspace(workspace: String) -> AppResponse<String> {
    let home = home_dir().unwrap();
    let base_path = Path::join(home.as_path(), ROOT_PATH)
        .join(WORKSPACE_PATH)
        .join(workspace.to_string());
    if base_path.exists() {
        fs::remove_dir(base_path).unwrap();
    }
    AppResponse {
        code: RESPONSE_CODE_SUCCESS,
        r#type: "".to_string(),
        message: "".to_string(),
        result: "".to_string(),
    }
}

#[tauri::command]
fn list_workspace_files(request: FileRequest) -> AppResponse<Vec<FileEntry>> {
    let mut response = AppResponse {
        code: RESPONSE_CODE_ERROR,
        r#type: "".to_string(),
        message: "".to_string(),
        result: vec![],
    };
    let home = home_dir().unwrap();
    let base_path = Path::join(home.as_path(), ROOT_PATH)
        .join(WORKSPACE_PATH)
        .join(request.workspace.clone());
    response.code = RESPONSE_CODE_SUCCESS;
    response.result = get_file_entry(
        base_path.to_str().unwrap(),
        request.path.as_str(),
        request.recursive,
        request.r#type,
        request.name,
    );
    return response;
}

#[tauri::command]
fn list_workspace_dirs(workspace: String) -> AppResponse<Vec<FileEntry>> {
    const DIR: &str = "dir";
    return match home_dir() {
        None => {
            return AppResponse {
                code: RESPONSE_CODE_ERROR,
                r#type: "".to_string(),
                message: "".to_string(),
                result: vec![],
            }
        }
        Some(home) => {
            let base_path = Path::join(home.as_path(), ROOT_PATH)
                .join(WORKSPACE_PATH)
                .join(workspace.clone());
            let root_metadata = fs::metadata(base_path.clone()).unwrap();

            let root = vec![FileEntry {
                name: "root".to_string(),
                r#type: DIR.to_string(),
                path: "".to_string(),
                parent_path: "".to_string(),
                children: get_file_entry(
                    base_path.to_str().unwrap(),
                    "",
                    true,
                    Some(DIR.to_string()),
                    None,
                ),
                size: root_metadata.size(),
                create_time: root_metadata
                    .created()
                    .unwrap()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
                modify_time: root_metadata
                    .modified()
                    .unwrap()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
            }];
            AppResponse {
                code: RESPONSE_CODE_SUCCESS,
                r#type: "".to_string(),
                message: "".to_string(),
                result: root,
            }
        }
    };
}

#[tauri::command]
fn create_workspace_file(
    workspace: String,
    file_name: String,
    file_type: String,
    parent_path: String,
) -> AppResponse<String> {
    match home_dir() {
        None => AppResponse {
            code: RESPONSE_CODE_ERROR,
            r#type: "".to_string(),
            message: "".to_string(),
            result: "".to_string(),
        },
        Some(home) => {
            let base_path = Path::join(home.as_path(), ROOT_PATH)
                .join(WORKSPACE_PATH)
                .join(workspace)
                .join(parent_path);
            if file_type == "dir" {
                fs::create_dir_all(base_path.join(file_name.clone())).unwrap();
            } else {
                File::create(base_path.join(file_name.clone())).unwrap();
            }
            AppResponse {
                code: RESPONSE_CODE_SUCCESS,
                r#type: "".to_string(),
                message: "".to_string(),
                result: file_name,
            }
        }
    }
}

#[tauri::command]
fn delete_workspace_file(
    workspace: String,
    file_name: String,
    parent_path: String,
) -> AppResponse<String> {
    match home_dir() {
        None => AppResponse {
            code: RESPONSE_CODE_ERROR,
            r#type: "".to_string(),
            message: "".to_string(),
            result: "".to_string(),
        },
        Some(home) => {
            let base_path = Path::join(home.as_path(), ROOT_PATH)
                .join(WORKSPACE_PATH)
                .join(workspace)
                .join(parent_path)
                .join(file_name.clone());
            match fs::metadata(base_path.clone()) {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        // remove all include file in dir
                        fs::remove_dir_all(base_path).unwrap()
                    } else {
                        fs::remove_file(base_path).unwrap()
                    }
                    AppResponse {
                        code: RESPONSE_CODE_SUCCESS,
                        r#type: "".to_string(),
                        message: "".to_string(),
                        result: file_name,
                    }
                }
                Err(e) => AppResponse {
                    code: RESPONSE_CODE_ERROR,
                    r#type: "".to_string(),
                    message: e.to_string(),
                    result: "".to_string(),
                },
            }
        }
    }
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
                if(option_name_filter.is_none()) {
                    file_entrys.push(file_entry)
                } else {
                    let name_filter = option_name_filter.clone().unwrap();
                    if file_name.find(name_filter.as_str()).is_some(){
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

#[tauri::command]
async fn ollama_get_models() -> AppResponse<ModelData> {
    let client = ClientBuilder::new().build().unwrap();
    let request = HttpRequestBuilder::new("GET", "http://localhost:11434/api/tags")
        .unwrap()
        .response_type(ResponseType::Json);
    if let Ok(response) = client.send(request).await {
        let data = response.read().await.unwrap().data;
        // println!("{}",data);
        let model_data: ModelData = serde_json::from_value(data).unwrap();
        return AppResponse {
            code: RESPONSE_CODE_SUCCESS,
            r#type: "".to_string(),
            message: "".to_string(),
            result: model_data,
        };
    }
    let model_data = ModelData { models: vec![] };
    return AppResponse {
        code: RESPONSE_CODE_SUCCESS,
        r#type: "".to_string(),
        message: "".to_string(),
        result: model_data,
    };
}

fn main() {
    // show banner
    banner();

    // process args
    let args: Args = Args::parse();

    let level = match args.verbose {
        0 => "info",
        1 => "debug",
        2 => "trace",
        _ => "",
    };

    // init tracing
    tracing_subscriber::fmt()
        .pretty()
        .with_line_number(false)
        .with_file(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_env_filter(level)
        .init();

    // init default path
    match home_dir() {
        None => {
            println!("Home directory not found.");
            exit(1)
        }
        Some(home) => {
            println!("Home directory path: {}", home.display());
            let root = Path::join(home.as_path(), ROOT_PATH);
            if !root.exists() {
                fs::create_dir(root.as_path()).unwrap();
                // create config
                fs::create_dir(root.join(CONFIG_PATH)).unwrap();
                // create workspace
                fs::create_dir_all(root.join(WORKSPACE_PATH).join(DEFAULT_WORKSPACE)).unwrap();
            }
        }
    }

    // process config
    let mut config_builder = config::Config::builder();

    config_builder = match &args.config {
        Some(config) => config_builder.add_source(config::File::with_name(config)),
        None => {
            info!("System use build-in config");
            config_builder.add_source(config::File::from_str(DEFAULT_CONFIG, FileFormat::Toml))
        }
    };
    let config: Config = config_builder.build().unwrap().try_deserialize().unwrap();

    tauri::Builder::default()
        .setup(|_app| {
            // start api
            let api_settings = config.api.unwrap();
            let mut api = Api::new(api_settings);
            let _api_handle = api.start().unwrap();
            // Ok(api_handle.join().unwrap())
            Ok(())
        })
        // why sync fn must after sync fc
        .invoke_handler(tauri::generate_handler![
            my_custom_command,
            list_workspaces,
            create_workspace,
            delete_workspace,
            list_workspace_dirs,
            list_workspace_files,
            create_workspace_file,
            delete_workspace_file,
            ollama_get_models
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use app::{FileRequest, FILE_TYPE};

    use crate::{
        create_workspace, create_workspace_file, delete_workspace, delete_workspace_file,
        list_workspace_files, list_workspaces,
    };

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ModelDataNew {
        pub models: String,
    }

    #[test] //由此判断这是一个测试函数
    fn it_works() {
        // let results = get_images();
        // assert_eq!(2, results.len())
        assert_eq!(true, "abc.txt".find("a").is_some())
    }

    #[test] //由此判断这是一个测试函数
    fn test_list_files() {
        let workspace = "test";

        create_workspace(workspace.to_string());

        let workspace_response = list_workspaces();
        for name in workspace_response.result {
            println!("workspace:{:?}", name)
        }

        // todo create workspace & files first
        create_workspace_file(
            workspace.to_string(),
            "file1.md".to_string(),
            "file".to_string(),
            "".to_string(),
        );
        create_workspace_file(
            workspace.to_string(),
            "dir1".to_string(),
            "dir".to_string(),
            "".to_string(),
        );
        create_workspace_file(
            workspace.to_string(),
            "a.svg".to_string(),
            "file".to_string(),
            "dir1".to_string(),
        );
        create_workspace_file(
            workspace.to_string(),
            "b.png".to_string(),
            "file".to_string(),
            "dir1".to_string(),
        );

        let request1 = FileRequest {
            workspace: workspace.to_string(),
            path: "".to_string(),
            name: None,
            r#type: None,
            recursive: false,
        };

        let response = list_workspace_files(request1);
        for file_entry in response.result {
            println!("r1:{:?}", file_entry)
        }

        let request2 = FileRequest {
            workspace: workspace.to_string(),
            path: "".to_string(),
            name: None,
            r#type: None,
            recursive: true,
        };

        let response = list_workspace_files(request2);
        for file_entry in response.result {
            println!("r2:{:?}", file_entry)
        }

        let request3 = FileRequest {
            workspace: workspace.to_string(),
            path: "".to_string(),
            name: None,
            r#type: Option::Some(FILE_TYPE.to_string()),
            recursive: true,
        };

        let response = list_workspace_files(request3);
        for file_entry in response.result {
            println!("r3:{:?}", file_entry)
        }

        delete_workspace_file(
            workspace.to_string(),
            "file1.md".to_string(),
            "".to_string(),
        );
        delete_workspace_file(workspace.to_string(), "dir1".to_string(), "".to_string());

        let response1 = delete_workspace(workspace.to_string());
        println!("{}", response1.result);
    }
}
