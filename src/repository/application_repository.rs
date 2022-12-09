use sqlx::PgPool;

use crate::{model::{Pagination, PaginationResponse, Application}, exception::{ApiError, APP_ERR_FINDING_PAGINATED_ERROR}};

pub struct ApplicationRepository;

impl ApplicationRepository {
    pub async fn find_application(pagination: Pagination, pg_pool: &PgPool) -> Result<PaginationResponse<Application>, ApiError> {
        let total = sqlx::query_scalar("select count(*) as count from anothergtw.tb_application")
            .fetch_one(pg_pool)
            .await
            .map_err(|e| {
                tracing::error!("Error when finding applications: {}", e);
                return ApiError::new(APP_ERR_FINDING_PAGINATED_ERROR);
            })?;

        let mut response = PaginationResponse {
            page: pagination.page.unwrap(),
            page_size: pagination.page_size.unwrap(),
            total,
            elements: Vec::new()
        };

        if total > 0 {
            let applications = sqlx::query_as!(Application, 
                r#"select * from anothergtw.tb_application limit $1 offset $2"#, 
                pagination.page_size.unwrap(), 
                pagination.offset()
            )
            .fetch_all(pg_pool)
            .await
            .map_err(|e| {
                tracing::error!("Error when finding applications: {}", e);
                return ApiError::new(APP_ERR_FINDING_PAGINATED_ERROR);
            })?;

            response.elements = applications;
        }       

        Ok(response)
    }
}