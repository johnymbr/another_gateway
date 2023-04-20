use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::exception::{ApiError, ApiFieldError, ERR_INVALID_REQUEST, ERR_REQUIRED_FIELD, ERR_MIN_SIZE};

#[derive(Serialize, Deserialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationReq {
    pub name: Option<String>,
    pub path: Option<String>,
    pub url_destination: Option<String>,
}

impl ApplicationReq {
    pub fn validate(&self) -> Result<(), ApiError> {
        let mut field_errors = Vec::<ApiFieldError>::new();

        if let Err(error) = self.validate_name(true) {
            field_errors.push(error);
        }

        if let Err(error) = self.validate_path(true) {
            field_errors.push(error);
        }

        if let Err(error) = self.validate_url_destination(true) {
            field_errors.push(error);
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

        if let Err(error) = self.validate_name(false) {
            field_errors.push(error);
        }

        if let Err(error) = self.validate_path(false) {
            field_errors.push(error);
        }

        if let Err(error) = self.validate_url_destination(false) {
            field_errors.push(error);
        }

        if !field_errors.is_empty() {
            return Err(ApiError::new_with_field_errors(
                ERR_INVALID_REQUEST,
                field_errors,
            ));
        }

        Ok(())
    }

    fn validate_name(&self, is_required: bool) -> Result<(), ApiFieldError> {
        match &self.name {
            Some(name) => {
                if name.len() < 3 {
                    Err(ApiFieldError::new_with_min_size(
                        ERR_MIN_SIZE,
                        "application.name".to_owned(),
                        3,
                    ))
                } else {
                    Ok(())
                }
                
            }
            None => {
                if is_required {
                    Err(ApiFieldError::new(
                        ERR_REQUIRED_FIELD,
                        "application.name".to_owned(),
                    ))
                } else {
                    Ok(())
                }
            }
        }
    }

    fn validate_path(&self, is_required: bool) -> Result<(), ApiFieldError> {
        match &self.path {
            Some(path) => {
                if path.len() < 3 {
                    Err(ApiFieldError::new_with_min_size(
                        ERR_MIN_SIZE,
                        "application.path".to_owned(),
                        3,
                    ))
                } else {
                    Ok(())
                }
            }
            None => {
                if is_required {
                    Err(ApiFieldError::new(
                        ERR_REQUIRED_FIELD,
                        "application.path".to_owned(),
                    ))
                } else {
                    Ok(())
                }
            }
        }
    }

    fn validate_url_destination(&self, is_required: bool) -> Result<(), ApiFieldError> {
        match &self.url_destination {
            Some(url_destination) => {
                if url_destination.len() < 3 {
                    Err(ApiFieldError::new_with_min_size(
                        ERR_MIN_SIZE,
                        "application.urlDestination".to_owned(),
                        3
                    ))
                } else {
                    Ok(())
                }
            }
            None => {
                if is_required {
                    Err(ApiFieldError::new(
                        ERR_REQUIRED_FIELD,
                        "application.urlDestination".to_owned(),
                    ))
                } else {
                    Ok(())
                }
            }
        }
    }
}
