use std::sync::Arc;

use axum::{
    extract::{self, Path, Query, State},
    response::IntoResponse,
    routing::{get},
    Json, Router,
};
use hyper::StatusCode;
use sqlx::PgPool;

use crate::{
    exception::ApiError,
    model::{ApplicationReq, Pagination},
    service::ApplicationService,
};

pub struct ApplicationController;

impl ApplicationController {
    pub fn routes(pg_pool: Arc<PgPool>) -> Router {
        Router::new()
            .route(
                "/application",
                get(ApplicationController::find_all).post(ApplicationController::save),
            )
            .route("/application/:id", get(ApplicationController::find_by_id))
            .with_state(pg_pool)
    }

    async fn find_all(
        Query(pagination): Query<Pagination>,
        State(pg_pool): State<Arc<PgPool>>,
    ) -> Result<impl IntoResponse, ApiError> {
        let response = ApplicationService::find_all(pagination, &pg_pool).await?;
        Ok((StatusCode::OK, Json(response)))
    }

    async fn find_by_id(
        Path(id): Path<i64>,
        State(pg_pool): State<Arc<PgPool>>,
    ) -> Result<impl IntoResponse, ApiError> {
        let response = ApplicationService::find_by_id(id, &pg_pool).await?;
        Ok((StatusCode::OK, Json(response)))
    }

    async fn save(
        State(pg_pool): State<Arc<PgPool>>,
        extract::Json(entity): extract::Json<ApplicationReq>,
    ) -> Result<impl IntoResponse, ApiError> {
        let response = ApplicationService::save(entity, &pg_pool).await?;
        Ok((StatusCode::OK, Json(response)))
    }
}
