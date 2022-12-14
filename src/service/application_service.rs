use std::sync::Arc;

use axum::async_trait;
use hyper::StatusCode;
use sqlx::PgPool;

use crate::{
    exception::{
        ApiError, ApiFieldError, APP_ERR_NOT_FOUND, ERR_INVALID_REQUEST, ERR_MIN_SIZE,
        ERR_REQUIRED_FIELD,
    },
    model::{Application, ApplicationReq, Pagination, PaginationResponse},
    repository::{ApplicationRepository, ApplicationRepositoryTrait},
};

#[async_trait]
pub trait ApplicationServiceTrait {
    async fn find_all(
        &self,
        pagination: Pagination
    ) -> Result<PaginationResponse<Application>, ApiError>;

    async fn find_by_id(&self, id: i64) -> Result<Application, ApiError>;

    async fn save(
        &self,
        entity: ApplicationReq
    ) -> Result<Application, ApiError>;
}

pub struct ApplicationService {
    application_repository: Arc<dyn ApplicationRepositoryTrait + Send + Sync>,
}

#[async_trait]
impl ApplicationServiceTrait for ApplicationService {
    async fn find_all(
        &self,
        pagination: Pagination
    ) -> Result<PaginationResponse<Application>, ApiError> {
        pagination.validate()?;

        let response = self
            .application_repository
            .find_all(pagination)
            .await?;
        Ok(response)
    }

    async fn find_by_id(&self, id: i64) -> Result<Application, ApiError> {
        let response = self.application_repository.find_by_id(id).await?;

        if let None = response {
            return Err(ApiError::new_with_status(
                StatusCode::NOT_FOUND,
                APP_ERR_NOT_FOUND,
            ));
        }
        Ok(response.unwrap())
    }

    async fn save(
        &self,
        entity: ApplicationReq
    ) -> Result<Application, ApiError> {
        let mut field_errors = Vec::<ApiFieldError>::new();

        if let Some(field_error) = self.validate_name(&entity) {
            field_errors.push(field_error);
        }

        if let Some(field_error) = self.validate_path(&entity) {
            field_errors.push(field_error);
        }

        if let Some(field_error) = self.validate_url_destination(&entity) {
            field_errors.push(field_error);
        }

        if !field_errors.is_empty() {
            return Err(ApiError::new(ERR_INVALID_REQUEST));
        }

        let application = self.application_repository.save(entity).await?;
        Ok(application)
    }
}

impl ApplicationService {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        ApplicationService {
            application_repository: Arc::new(ApplicationRepository {
                pg_pool
            }),
        }
    }

    pub fn new_with_repo(repository: Arc<dyn ApplicationRepositoryTrait + Send + Sync>) -> Self {
        ApplicationService {
            application_repository: repository,
        }
    }

    fn validate_name(&self, entity: &ApplicationReq) -> Option<ApiFieldError> {
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

    fn validate_path(&self, entity: &ApplicationReq) -> Option<ApiFieldError> {
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

    fn validate_url_destination(&self, entity: &ApplicationReq) -> Option<ApiFieldError> {
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
