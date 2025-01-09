use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CopyBody {
    pub from_id: String,
    pub name: String,
    pub pid: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateBody {
    pub name: String,
    pub pid: String,
    pub wid: String,
    pub r#type: String,
    #[serde(default = "default_zone")]
    pub zone: String,
    pub content: Option<String>,
    pub path: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ListGeneralBody {
    pub wid: String,
    #[serde(default = "default_zone")]
    pub zone: String,
    pub r#type: Option<String>,
}

fn default_zone() -> String {
    "".to_string()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ListByPageBody {
    pub page_size: u64,
    pub page_num: u64,
    pub pid: String,
    pub r#type: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateContentBody {
    pub id: String,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBody {
    pub id: String,
    pub name: String,
    pub pid: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNameBody {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ListByPidBody {
    pub pid: String,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GeneralBody {
    pub wid: String,
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PageResult {
    pub total: u64,
    pub items: Vec<crate::entity::file::Model>,
}
