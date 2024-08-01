use axum::Json;
use serde::{Deserialize, Serialize};

use crate::{AppResponse, RESPONSE_CODE_SUCCESS};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
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
pub struct Role {
    role_name: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    path: String,
    name: String,
    component: String,
    redirect: String,
    meta: Meta,
    children: Vec<ChildRoute>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    title: String,
    hide_children_in_menu: bool,
    icon: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChildRoute {
    path: String,
    name: String,
    component: String,
    meta: ChildMeta,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChildMeta {
    hide_menu: bool,
    hide_breadcrumb: bool,
    title: String,
    current_active_menu: String,
    icon: String,
}

pub async fn login() -> Json<AppResponse<UserInfo>> {
    get_user_info().await
}

pub async fn logout() -> Json<AppResponse<String>> {
    let resp = AppResponse {
        code: RESPONSE_CODE_SUCCESS,
        r#type: String::from(""),
        message: String::from(""),
        result: String::from(""),
    };
    Json(resp)
}

pub async fn get_perm_code() -> Json<AppResponse<Vec<String>>> {
    Json(AppResponse {
        code: RESPONSE_CODE_SUCCESS,
        r#type: "".to_string(),
        message: "".to_string(),
        result: vec![
            String::from("1000"),
            String::from("3000"),
            String::from("5000"),
        ],
    })
}

pub async fn get_user_info() -> Json<AppResponse<UserInfo>> {
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

    let resp = AppResponse {
        code: RESPONSE_CODE_SUCCESS,
        r#type: "".to_string(),
        message: "".to_string(),
        result: user_info,
    };
    Json(resp)
}

pub async fn get_menu_list() -> Json<AppResponse<Vec<Route>>> {
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

    Json(AppResponse {
        code: RESPONSE_CODE_SUCCESS,
        r#type: "".to_string(),
        message: "".to_string(),
        result: vec![dashboard_route],
    })
}
