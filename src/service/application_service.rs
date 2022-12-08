use axum::extract::State;
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use sqlx::PgPool;

use crate::{
    exception::{
        ApiError, ApiErrorCode, ApiFieldError, APP_ERR_INSERTING_ERROR, ERR_INVALID_REQUEST,
        ERR_MIN_SIZE, ERR_REQUIRED_FIELD,
    },
    model::{Application, ApplicationReq, Pagination, PaginationResponse},
};

pub struct ApplicationService;

impl ApplicationService {
    pub async fn find_application(
        pagination: Pagination,
        pg_pool: &PgPool,
    ) -> Result<PaginationResponse<Application>, ApiError> {

        let total = sqlx::query_scalar("select count(*) as count from anothergateway.tb_application")
            .fetch_one(pg_pool)
            .await
            .map_err(|e| {
                tracing::info!("Error when inserting an application: {}", e);
                return ApiError::new(APP_ERR_INSERTING_ERROR);
            })?;

        let response = PaginationResponse {
            page: pagination.page,
            page_size: pagination.page_size,
            total,
            elements: Vec::new()
        };

        Ok(response)
    }

    pub async fn save(entity: ApplicationReq, pool: &PgPool) -> Result<Application, ApiError> {
        let mut field_errors = Vec::<ApiFieldError>::new();

        if let Some(field_error) = ApplicationService::validate_name(&entity) {
            field_errors.push(field_error);
        }

        // TODO: validate other fields.

        if !field_errors.is_empty() {
            return Err(ApiError::new(ERR_INVALID_REQUEST));
        }

        let application = sqlx::query_as("insert into anothergateway.tb_applicaiton(name, path, url_destination, created_dttm, update_dttm) values ($1, $2, $3, $4, $5) returning *;")
            .bind(entity.name)
            .bind(entity.path)
            .bind(entity.url_destination)
            .bind(Utc::now())
            .bind(Utc::now())
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::info!("Error when inserting an application: {}", e);
                return ApiError::new(APP_ERR_INSERTING_ERROR);
            })?;

        Ok(application)
    }

    fn validate_name(entity: &ApplicationReq) -> Option<ApiFieldError> {
        match entity.name.to_owned() {
            Some(name) => {
                if name.len() < 3 {
                    Some(ApiFieldError::new_with_min_size(
                        ERR_MIN_SIZE,
                        "application.name".to_owned(),
                        3,
                    ))
                } else {
                    None
                }
            }
            None => Some(ApiFieldError::new(
                ERR_REQUIRED_FIELD,
                "application.name".to_owned(),
            )),
        }
    }
}
