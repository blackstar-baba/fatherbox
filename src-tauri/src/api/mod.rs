pub mod auth;
pub mod file;

use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use axum::http::Method;
use axum::Router;
use axum::routing::{get, post};

use log::info;
use tokio::runtime;
use tower_http::cors::CorsLayer;

use crate::api::auth::{get_menu_list, get_perm_code, get_user_info, login, logout};
use crate::api::file::{download_file, upload_file};
use crate::{ApiSettings, AppResponse, RESPONSE_CODE_SUCCESS};

pub struct Api {
    settings: ApiSettings,
}

impl Api {
    pub fn new(settings: ApiSettings) -> Api {
        Api { settings }
    }

    pub fn start(&mut self) -> anyhow::Result<JoinHandle<()>> {
        //todo how to get abs path
        let shared_path = Arc::new(PathBuf::from("/Users/blackstar/.fatherbox/workspace"));
        let cors = CorsLayer::new()
            // allow `GET` and `POST` when accessing the resource
            .allow_methods([Method::GET, Method::POST])
            // allow requests from any origin
            .allow_origin(tower_http::cors::Any);
        let server_thread = thread::Builder::new().name(String::from("api"));
        let server_url = self.settings.listen.clone();
        let handle = server_thread.spawn(move || {
            let mut runtime = runtime::Builder::new_current_thread();
            let runtime = runtime.enable_all().build().unwrap();
            runtime.block_on(async {
                let app = Router::new()
                    .route("/login", post(login))
                    .route("/getUserInfo", get(get_user_info))
                    .route("/getPermCode", get(get_perm_code))
                    .route("/getMenuList", get(get_menu_list))
                    .route("/file/:fileName", get(download_file))
                    .route("/file", post(upload_file))
                    .route("/logout", post(logout))
                    .layer(cors);
                let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
                info!("start api server on {}", server_url);
                axum::serve(listener, app).await.unwrap();
            });
        })?;
        Ok(handle)
    }
}
