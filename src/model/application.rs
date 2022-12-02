use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Application {
    pub id: i64,
    pub name: String,
    pub path: String,
    #[serde(rename = "urlDestination")]
    pub url_destination: String,
    #[serde(rename = "createdDttm")]
    pub created_dttm: DateTime<Utc>,
    #[serde(rename = "updateDttm")]
    pub update_dttm: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationReq {
    pub name: Option<String>,
    pub path: Option<String>,
    #[serde(rename = "urlDestination")]
    pub url_destination: Option<String>,
}
