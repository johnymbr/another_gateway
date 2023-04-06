#[cfg(test)]
#[path = "application_service_test.rs"]
mod application_service_test;

use std::sync::Arc;

use axum::async_trait;
use hyper::StatusCode;
use sqlx::PgPool;

use crate::{
    exception::{ApiError, APP_ERR_NOT_FOUND},
    model::{Application, ApplicationReq, Pagination, PaginationResponse},
    repository::{ApplicationRepository, ApplicationRepositoryTrait},
};

#[async_trait]
pub trait ApplicationServiceTrait {
    async fn find_all(
        &self,
        pagination: Pagination,
    ) -> Result<PaginationResponse<Application>, ApiError>;

    async fn find_by_id(&self, id: i64) -> Result<Application, ApiError>;

    async fn find_by_path(&self, path: &str) -> Result<Application, ApiError>;

    async fn save(&self, entity: ApplicationReq) -> Result<Application, ApiError>;

    async fn update(&self, id: i64, entity: ApplicationReq) -> Result<Application, ApiError>;

    async fn delete(&self, id: i64) -> Result<(), ApiError>;
}

pub struct ApplicationService {
    application_repository: Arc<dyn ApplicationRepositoryTrait + Send + Sync>,
}

#[async_trait]
impl ApplicationServiceTrait for ApplicationService {
    async fn find_all(
        &self,
        pagination: Pagination,
    ) -> Result<PaginationResponse<Application>, ApiError> {
        pagination.validate()?;

        let response = self.application_repository.find_all(pagination).await?;
        Ok(response)
    }

    async fn find_by_id(&self, id: i64) -> Result<Application, ApiError> {
        let response = self.application_repository.find_by_id(id).await?;

        if response.is_none() {
            return Err(ApiError::new_with_status(
                StatusCode::NOT_FOUND,
                APP_ERR_NOT_FOUND,
            ));
        }
        Ok(response.unwrap())
    }

    async fn find_by_path(&self, path: &str) -> Result<Application, ApiError> {
        let response = self.application_repository.find_by_path(path).await?;

        if response.is_none() {
            return Err(ApiError::new_with_status(StatusCode::NOT_FOUND, APP_ERR_NOT_FOUND))
        }

        Ok(response.unwrap())
    }

    async fn save(&self, entity: ApplicationReq) -> Result<Application, ApiError> {
        entity.validate()?;

        let application = self.application_repository.save(entity).await?;
        Ok(application)
    }

    async fn update(&self, id: i64, entity: ApplicationReq) -> Result<Application, ApiError> {
        entity.validate_updating()?;

        if let Some(mut application) = self.application_repository.find_by_id(id).await? {
            if let Some(name) = entity.name {
                application.name = name.value();
            }

            if let Some(path) = entity.path {
                application.path = path.value();
            }

            if let Some(url_destination) = entity.url_destination {
                application.url_destination = url_destination.value();
            }

            application = self.application_repository.update(application).await?;
            Ok(application)
        } else {
            Err(ApiError::new_with_status(
                StatusCode::NOT_FOUND,
                APP_ERR_NOT_FOUND,
            ))
        }
    }

    async fn delete(&self, id: i64) -> Result<(), ApiError> {
        if (self.application_repository.find_by_id(id).await?).is_some() {
            self.application_repository.delete(id).await?;
            Ok(())
        } else {
            Err(ApiError::new_with_status(
                StatusCode::NOT_FOUND,
                APP_ERR_NOT_FOUND,
            ))
        }
    }
}

impl ApplicationService {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        ApplicationService {
            application_repository: Arc::new(ApplicationRepository { pg_pool }),
        }
    }

    pub fn new_with_repo(repository: Arc<dyn ApplicationRepositoryTrait + Send + Sync>) -> Self {
        ApplicationService {
            application_repository: repository,
        }
    }
}
