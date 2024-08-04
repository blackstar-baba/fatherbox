// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::SystemTime;
use std::{env, fs};

use clap::Parser;
use config::FileFormat;
use log::{error, info};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Schema};
use tauri::api::http::{ClientBuilder, HttpRequestBuilder, ResponseType};
use tauri::api::path::home_dir;

use crate::db_utils::init_connection;
use crate::file_command::{
    create_workspace_dir_cmd, create_workspace_file_cmd, create_workspace_file_inner,
    delete_workspace_file_cmd, list_workspace_dirs_cmd, list_workspace_files_cmd,
    update_workspace_dir_cmd, update_workspace_file_cmd,
};
use crate::model_command::ollama_get_models_cmd;
use crate::workspace_command::{
    create_workspace_cmd, create_workspace_inner, delete_workspace_cmd, get_workspace_inner,
    list_workspaces_cmd,
};
use app::api::{file, Api};
use app::service::workspace_service::WorkspaceService;
use app::{
    entity, AppResponse, AppState, Config, FileEntry, FileRequest, ModelData, CONFIG_PATH,
    DEFAULT_WORKSPACE, DIR_TYPE, FILE_TYPE, RESPONSE_CODE_ERROR, RESPONSE_CODE_SUCCESS, ROOT_PATH,
    WORKSPACE_PATH,
};

mod db_utils;
mod file_command;
mod model_command;
mod workspace_command;

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

#[tokio::main]
async fn main() {
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
    if home_dir().is_none() {
        error!("Home directory not found.");
        exit(1)
    }
    let home = home_dir().unwrap();
    info!("Home directory path: {}", home.display());
    let root_path = &home.join(ROOT_PATH);
    if !root_path.exists() {
        info!("Create root path: {}", root_path.display());
        fs::create_dir(root_path.as_path()).unwrap();
    }
    let config_path = &root_path.join(CONFIG_PATH);
    if !config_path.exists() {
        // create config
        info!("Create config path: {}", config_path.display());
        fs::create_dir(config_path).unwrap();
    }
    let workspace_path = &root_path.join(WORKSPACE_PATH);
    if !workspace_path.exists() {
        // create workspace
        info!("Create workspace path: {}", workspace_path.display());
        fs::create_dir(workspace_path).unwrap();
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

    // process db
    let db_file_path = home_dir().unwrap().join(ROOT_PATH).join("data.sqlite");
    info!("begin init db use file {:?}", db_file_path);
    let db_exist = db_file_path.exists();
    let db = match init_connection(&db_file_path).await {
        Ok(conn) => conn,
        Err(err) => {
            info!("init connection catch err: {:?}", err);
            exit(1)
        }
    };
    if !db_exist {
        info!("begin init tables in db");
        let builder = db.get_database_backend();
        let schema = Schema::new(builder);
        db.execute(builder.build(&schema.create_table_from_entity(entity::prelude::Workspace)))
            .await
            .unwrap();
        db.execute(builder.build(&schema.create_table_from_entity(entity::prelude::File)))
            .await
            .unwrap();
    }
    let option_workspace_model = WorkspaceService::get_workspace_by_name(&db, DEFAULT_WORKSPACE)
        .await
        .unwrap();
    if option_workspace_model.is_none() {
        // create dir
        let default_workspace_path = &workspace_path.join(DEFAULT_WORKSPACE);
        if !default_workspace_path.exists() {
            // create workspace
            info!(
                "Create default workspace: {}",
                default_workspace_path.display()
            );
            fs::create_dir(default_workspace_path).unwrap();
        }
        // create default workspace
        let workspace = create_workspace_inner(&db, DEFAULT_WORKSPACE.to_string())
            .await
            .expect("create default workspace failed")
            .result;
        // create default workspace dir
        let _ = create_workspace_file_inner(&db, &workspace.id, "", DIR_TYPE, DEFAULT_WORKSPACE)
            .await
            .expect("create default workspace dir failed");
    }

    tauri::Builder::default()
        .setup(|_app| {
            // start api
            let api_settings = config.api.unwrap();
            let mut api = Api::new(api_settings);
            let _api_handle = api.start().unwrap();
            // Ok(api_handle.join().unwrap())
            Ok(())
        })
        .manage(AppState {
            conn: db,
            root_path: root_path.to_owned(),
            workspace_path: workspace_path.to_owned(),
        })
        // why sync fn must after sync fc
        .invoke_handler(tauri::generate_handler![
            my_custom_command,
            list_workspaces_cmd,
            create_workspace_cmd,
            delete_workspace_cmd,
            list_workspace_dirs_cmd,
            list_workspace_files_cmd,
            create_workspace_dir_cmd,
            create_workspace_file_cmd,
            update_workspace_dir_cmd,
            update_workspace_file_cmd,
            delete_workspace_file_cmd,
            ollama_get_models_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use sea_orm::EntityTrait;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ModelDataNew {
        pub models: String,
    }

    #[test] //由此判断这是一个测试函数
    fn it_works() {
        // let results = get_images();
        // assert_eq!(2, results.len())
        assert_eq!(true, "abc.txt".find("a").is_some());
    }
}
