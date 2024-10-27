pub mod api;
pub mod entity;
pub mod service;
pub mod adapter;
pub mod util;

use std::fs;
use std::fs::File;
use sea_orm::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use anyhow::{anyhow, Error};
use tauri::api::path::home_dir;
use thiserror::Error;
use crate::util::db_util::exist_database_file;

pub const ROOT_PATH: &str = ".fatherbox";
pub const CONFIG_PATH: &str = "config";
pub const WORKSPACE_PATH: &str = "workspace";
pub const DEFAULT_WORKSPACE: &str = "default";

pub const FILE_TYPE: &str = "file";
pub const DIR_TYPE: &str = "dir";

pub const RESPONSE_CODE_SUCCESS: i32 = 0;
pub const RESPONSE_CODE_ERROR: i32 = -1;
pub const RESPONSE_CODE_TIMEOUT: i32 = 401;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub api: Option<ApiSettings>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiSettings {
    pub listen: SocketAddr,
    pub db_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppResponse<T> {
    pub code: i32,
    pub r#type: String,
    pub message: String,
    pub result: T,
}

impl<T> AppResponse<T> {
    pub fn success(result: T) -> Self {
        Self {
            code: RESPONSE_CODE_SUCCESS,
            r#type: String::new(),
            message: String::new(),
            result,
        }
    }

    pub fn error(result: T, err_message: &str) -> Self {
        Self {
            code: RESPONSE_CODE_ERROR,
            r#type: String::new(),
            message: err_message.to_string(),
            result,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    pub name: String,
    pub r#type: String,
    pub path: String,
    pub parent_path: String,
    pub children: Vec<FileEntry>,
    pub size: u64,
    pub create_time: u128,
    pub modify_time: u128,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileRequest {
    pub workspace_id: String,
    pub parent_id: String,
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub recursive: bool,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub root_path: PathBuf,
    pub workspace_path: PathBuf,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("An IO error occurred: {0}")]
    Io(#[from] std::io::Error),

    #[error("A parse error occurred: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("An unknown error occurred")]
    Unknown,
}

pub fn get_work_space_path(work_space_name: &str) -> String {
    home_dir()
        .unwrap()
        .join(ROOT_PATH)
        .join(WORKSPACE_PATH)
        .join(work_space_name)
        .to_str()
        .unwrap()
        .to_string()
}
