use std::thread;
use std::thread::JoinHandle;

use axum::routing::{get, post};
use axum::{Json, Router};
use log::info;
use serde::{Deserialize, Serialize};
use tokio::runtime;

use crate::{ApiSettings, Response, RESPONSE_CODE_SUCCESS};

pub struct Api {
    settings: ApiSettings,
}

impl Api {
    pub fn new(settings: ApiSettings) -> Api {
        Api { settings }
    }

    pub fn start(&mut self) -> anyhow::Result<JoinHandle<()>> {
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
                    .route("/logout", get(logout));
                let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
                info!("start api server on {}", server_url);
                axum::serve(listener, app).await.unwrap();
            });
        })?;
        Ok(handle)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct UserInfo {
    user_id: String,
    username: String,
    real_name: String,
    avatar: String,
    desc: String,
    password: String,
    token: String,
    home_path: String,
    roles: Vec<Role>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Role {
    role_name: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Route {
    path: String,
    name: String,
    component: String,
    redirect: String,
    meta: Meta,
    children: Vec<ChildRoute>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Meta {
    title: String,
    hide_children_in_menu: bool,
    icon: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ChildRoute {
    path: String,
    name: String,
    component: String,
    meta: ChildMeta,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ChildMeta {
    hide_menu: bool,
    hide_breadcrumb: bool,
    title: String,
    current_active_menu: String,
    icon: String,
}

async fn login() -> Json<Response<UserInfo>> {
    get_user_info().await
}

async fn logout() -> Json<Response<String>> {
    let resp = Response {
        code: RESPONSE_CODE_SUCCESS,
        r#type: String::from(""),
        message: String::from(""),
        result: String::from(""),
    };
    Json(resp)
}

async fn get_perm_code() -> Json<Response<Vec<String>>> {
    Json(Response {
        code:  RESPONSE_CODE_SUCCESS,
        r#type: "".to_string(),
        message: "".to_string(),
        result: vec![
            String::from("1000"),
            String::from("3000"),
            String::from("5000"),
        ],
    })
}

async fn get_user_info() -> Json<Response<UserInfo>> {
    // Send the protected data to the user
    let json_data = r#"{
      "userId": "1",
      "username": "vben",
      "realName": "Vben Admin",
      "avatar": "",
      "desc": "manager",
      "password": "123456",
      "token": "fakeToken1",
      "homePath": "/dashboard/analysis",
      "roles": [
        {
          "roleName": "Super Admin",
          "value": "super"
        }
      ]
    }"#;

    let user_info: UserInfo = serde_json::from_str(json_data).unwrap();

    let resp = Response {
        code:  RESPONSE_CODE_SUCCESS,
        r#type: "".to_string(),
        message: "".to_string(),
        result: user_info,
    };
    Json(resp)
}

async fn get_menu_list() -> Json<Response<Vec<Route>>> {
    let dashboard_json_data = r#"{
            "path": "/dashboard",
            "name": "Dashboard",
            "component": "LAYOUT",
            "redirect": "/dashboard/analysis",
            "meta": {
            "title": "routes.dashboard.dashboard",
            "hideChildrenInMenu": true,
            "icon": "bx:bx-home"
            },
            "children": [
                {
                    "path": "analysis",
                    "name": "Analysis",
                    "component": "/dashboard/analysis/index",
                    "meta": {
                    "hideMenu": true,
                    "hideBreadcrumb": true,
                    "title": "routes.dashboard.analysis",
                    "currentActiveMenu": "/dashboard",
                    "icon": "bx:bx-home"
                    }
                },
                {
                    "path": "workbench",
                    "name": "Workbench",
                    "component": "/dashboard/workbench/index",
                    "meta": {
                    "hideMenu": true,
                    "hideBreadcrumb": true,
                    "title": "routes.dashboard.workbench",
                    "currentActiveMenu": "/dashboard",
                    "icon": "bx:bx-home"
                    }
                }
            ]
    }"#;

    let dashboard_route: Route = serde_json::from_str(dashboard_json_data).unwrap();

    Json(Response {
        code: RESPONSE_CODE_SUCCESS,
        r#type: "".to_string(),
        message: "".to_string(),
        result: vec![dashboard_route],
    })
}