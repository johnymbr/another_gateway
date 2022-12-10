use chrono::Utc;
use sqlx::PgPool;

use crate::{
    exception::{
        ApiError, APP_ERR_FINDING_PAGINATED, APP_ERR_FIND_BY_ID, APP_ERR_INSERTING,
    },
    model::{Application, ApplicationReq, Pagination, PaginationResponse},
};

pub struct ApplicationRepository;

impl ApplicationRepository {
    pub async fn find_all(
        pagination: Pagination,
        pg_pool: &PgPool,
    ) -> Result<PaginationResponse<Application>, ApiError> {
        let total = sqlx::query_scalar("select count(*) as count from anothergtw.tb_application")
            .fetch_one(pg_pool)
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
            .fetch_all(pg_pool)
            .await
            .map_err(|e| {
                tracing::error!("Error when finding applications: {}", e);
                return ApiError::new(APP_ERR_FINDING_PAGINATED);
            })?;

            response.elements = applications;
        }

        Ok(response)
    }

    pub async fn find_by_id(id: i64, pg_pool: &PgPool) -> Result<Option<Application>, ApiError> {
        let application = sqlx::query_as!(
            Application,
            r#"select * from anothergtw.tb_application where id = $1"#,
            id
        )
        .fetch_optional(pg_pool)
        .await
        .map_err(|e| {
            tracing::error!("Error when finding an application by id: {}", e);
            return ApiError::new(APP_ERR_FIND_BY_ID);
        })?;

        Ok(application)
    }

    pub async fn save(entity: ApplicationReq, pg_pool: &PgPool) -> Result<Application, ApiError> {
        let application = sqlx::query_as("insert into anothergtw.tb_applicaiton(name, path, url_destination, created_dttm, update_dttm) values ($1, $2, $3, $4, $5) returning *;")
            .bind(entity.name)
            .bind(entity.path)
            .bind(entity.url_destination)
            .bind(Utc::now())
            .bind(Utc::now())
            .fetch_one(pg_pool)
            .await
            .map_err(|e| {
                tracing::info!("Error when inserting an application: {}", e);
                return ApiError::new(APP_ERR_INSERTING);
            })?;

        Ok(application)
    }
}
