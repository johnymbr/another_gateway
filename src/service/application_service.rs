use std::sync::Arc;

use axum::async_trait;
use hyper::StatusCode;
use sqlx::PgPool;

use crate::{
    exception::{
        ApiError, APP_ERR_NOT_FOUND,
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
        entity.validate()?;

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
}
