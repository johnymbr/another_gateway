use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::exception::{ApiError, ApiFieldError, ERR_INVALID_REQUEST, ERR_REQUIRED_FIELD};

use super::StringMinSize3;

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
#[serde(rename_all = "camelCase")]
pub struct ApplicationReq {
    pub name: Option<StringMinSize3>,
    pub path: Option<StringMinSize3>,
    #[serde(rename = "urlDestination")]
    pub url_destination: Option<StringMinSize3>,
}

impl ApplicationReq {
    pub fn validate(&self) -> Result<(), ApiError> {
        let mut field_errors = Vec::<ApiFieldError>::new();

        match &self.name {
            Some(name) => {
                if let Err(e) = name.validate("application.name".to_owned()) {
                    field_errors.push(e);
                }
            }
            None => {
                field_errors.push(ApiFieldError::new(
                    ERR_REQUIRED_FIELD,
                    "application.name".to_owned(),
                ));
            }
        }

        match &self.path {
            Some(path) => {
                if let Err(e) = path.validate("application.path".to_owned()) {
                    field_errors.push(e);
                }
            }
            None => {
                field_errors.push(ApiFieldError::new(
                    ERR_REQUIRED_FIELD,
                    "application.path".to_owned(),
                ));
            }
        }

        match &self.url_destination {
            Some(url_destination) => {
                if let Err(e) = url_destination.validate("application.urlDestination".to_owned()) {
                    field_errors.push(e);
                }
            }
            None => {
                field_errors.push(ApiFieldError::new(
                    ERR_REQUIRED_FIELD,
                    "application.urlDestination".to_owned(),
                ));
            }
        }

        if !field_errors.is_empty() {
            return Err(ApiError::new_with_field_errors(
                ERR_INVALID_REQUEST,
                field_errors,
            ));
        }

        Ok(())
    }

    pub fn validate_updating(&self) -> Result<(), ApiError> {
        let mut field_errors = Vec::<ApiFieldError>::new();

        match &self.name {
            Some(name) => {
                if let Err(e) = name.validate("application.name".to_owned()) {
                    field_errors.push(e);
                }
            }
            None => {}
        }

        match &self.path {
            Some(path) => {
                if let Err(e) = path.validate("application.path".to_owned()) {
                    field_errors.push(e);
                }
            }
            None => {}
        }

        match &self.url_destination {
            Some(url_destination) => {
                if let Err(e) = url_destination.validate("application.urlDestination".to_owned()) {
                    field_errors.push(e);
                }
            }
            None => {}
        }

        if !field_errors.is_empty() {
            return Err(ApiError::new_with_field_errors(
                ERR_INVALID_REQUEST,
                field_errors,
            ));
        }

        Ok(())
    }
}
