use std::sync::Arc;

use axum::async_trait;
use chrono::Utc;
use sqlx::PgPool;

use crate::{
    exception::{ApiError, APP_ERR_FINDING_PAGINATED, APP_ERR_FIND_BY_ID, APP_ERR_INSERTING},
    model::{Application, ApplicationReq, Pagination, PaginationResponse},
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ApplicationRepositoryTrait {
    async fn find_all(&self,
        pagination: Pagination
    ) -> Result<PaginationResponse<Application>, ApiError>;

    async fn find_by_id(&self, id: i64) -> Result<Option<Application>, ApiError>;

    async fn save(&self, entity: ApplicationReq) -> Result<Application, ApiError>;
}

pub struct ApplicationRepository {
    pub pg_pool: Arc<PgPool>,
}

#[async_trait]
impl ApplicationRepositoryTrait for ApplicationRepository {
    async fn find_all(&self,
        pagination: Pagination
    ) -> Result<PaginationResponse<Application>, ApiError> {
        let total = sqlx::query_scalar("select count(*) as count from anothergtw.tb_application")
            .fetch_one(&*self.pg_pool)
            .await
            .map_err(|e| {
                tracing::error!("Error when finding applications: {}", e);
                return ApiError::new(APP_ERR_FINDING_PAGINATED);
            })?;

        let mut response = PaginationResponse {
            page: pagination.page.unwrap(),
            page_size: pagination.page_size.unwrap(),
            total,
            elements: Vec::new(),
        };

        if total > 0 {
            let applications = sqlx::query_as!(
                Application,
                r#"select * from anothergtw.tb_application limit $1 offset $2"#,
                pagination.page_size.unwrap(),
                pagination.offset()
            )
            .fetch_all(&*self.pg_pool)
            .await
            .map_err(|e| {
                tracing::error!("Error when finding applications: {}", e);
                return ApiError::new(APP_ERR_FINDING_PAGINATED);
            })?;

            response.elements = applications;
        }

        Ok(response)
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Application>, ApiError> {
        let application = sqlx::query_as!(
            Application,
            r#"select * from anothergtw.tb_application where id = $1"#,
            id
        )
        .fetch_optional(&*self.pg_pool)
        .await
        .map_err(|e| {
            tracing::error!("Error when finding an application by id: {}", e);
            return ApiError::new(APP_ERR_FIND_BY_ID);
        })?;

        Ok(application)
    }

    async fn save(&self, entity: ApplicationReq) -> Result<Application, ApiError> {
        let application = sqlx::query_as("insert into anothergtw.tb_applicaiton(name, path, url_destination, created_dttm, update_dttm) values ($1, $2, $3, $4, $5) returning *;")
            .bind(entity.name.unwrap().value())
            .bind(entity.path.unwrap().value())
            .bind(entity.url_destination.unwrap().value())
            .bind(Utc::now())
            .bind(Utc::now())
            .fetch_one(&*self.pg_pool)
            .await
            .map_err(|e| {
                tracing::info!("Error when inserting an application: {}", e);
                return ApiError::new(APP_ERR_INSERTING);
            })?;

        Ok(application)
    }
}
