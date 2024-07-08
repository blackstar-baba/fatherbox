pub mod api;

use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use tauri::api::path::home_dir;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelData {
    pub models: Vec<Model>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    name: String,
    model: String,
    modified_at: String,
    size: i64,
    digest: String,
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
pub struct FileRequest {
    pub workspace: String,
    pub path: String,
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub recursive: bool,
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
