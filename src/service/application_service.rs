use hyper::StatusCode;
use sqlx::PgPool;

use crate::{
    exception::{
        ApiError, ApiFieldError, APP_ERR_NOT_FOUND, ERR_INVALID_REQUEST, ERR_MIN_SIZE,
        ERR_REQUIRED_FIELD,
    },
    model::{Application, ApplicationReq, Pagination, PaginationResponse},
    repository::ApplicationRepository,
};

pub struct ApplicationService;

impl ApplicationService {
    pub async fn find_all(
        pagination: Pagination,
        pg_pool: &PgPool,
    ) -> Result<PaginationResponse<Application>, ApiError> {
        pagination.validate()?;

        let response = ApplicationRepository::find_all(pagination, pg_pool).await?;
        Ok(response)
    }

    pub async fn find_by_id(id: i64, pg_pool: &PgPool) -> Result<Application, ApiError> {
        let response = ApplicationRepository::find_by_id(id, pg_pool).await?;

        if let None = response {
            return Err(ApiError::new_with_status(
                StatusCode::NOT_FOUND,
                APP_ERR_NOT_FOUND,
            ));
        }
        Ok(response.unwrap())
    }

    pub async fn save(entity: ApplicationReq, pg_pool: &PgPool) -> Result<Application, ApiError> {
        let mut field_errors = Vec::<ApiFieldError>::new();

        if let Some(field_error) = ApplicationService::validate_name(&entity) {
            field_errors.push(field_error);
        }

        if let Some(field_error) = ApplicationService::validate_path(&entity) {
            field_errors.push(field_error);
        }

        if let Some(field_error) = ApplicationService::validate_url_destination(&entity) {
            field_errors.push(field_error);
        }

        if !field_errors.is_empty() {
            return Err(ApiError::new(ERR_INVALID_REQUEST));
        }

        let application = ApplicationRepository::save(entity, pg_pool).await?;
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

    fn validate_path(entity: &ApplicationReq) -> Option<ApiFieldError> {
        match entity.path.to_owned() {
            Some(path) => {
                if path.len() < 3 {
                    Some(ApiFieldError::new_with_min_size(
                        ERR_MIN_SIZE,
                        "application.path".to_owned(),
                        3,
                    ))
                } else {
                    None
                }
            }
            None => Some(ApiFieldError::new(
                ERR_REQUIRED_FIELD,
                "application.path".to_owned(),
            )),
        }
    }

    fn validate_url_destination(entity: &ApplicationReq) -> Option<ApiFieldError> {
        match entity.url_destination.to_owned() {
            Some(url_destination) => {
                if url_destination.len() < 3 {
                    Some(ApiFieldError::new_with_min_size(
                        ERR_MIN_SIZE,
                        "application.urlDestination".to_owned(),
                        3,
                    ))
                } else {
                    None
                }
            }
            None => Some(ApiFieldError::new(
                ERR_REQUIRED_FIELD,
                "application.urlDestination".to_owned(),
            )),
        }
    }
}
