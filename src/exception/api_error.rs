use core::fmt;

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    #[serde(rename = "status")]
    pub status_code: u16,
    pub code: String,
    pub message: String,
    #[serde(rename = "fieldErrors")]
    pub field_errors: Option<Vec<ApiFieldError>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiFieldError {
    pub code: String,
    pub message: String,
    pub field: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(format!("{} - {}", self.code, self.message).as_str())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, Json(self)).into_response()
    }
}
