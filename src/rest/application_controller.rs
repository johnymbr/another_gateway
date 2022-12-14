use std::sync::Arc;

use axum::{
    extract::{self, Path, Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use hyper::StatusCode;
use sqlx::PgPool;

use crate::{
    exception::ApiError,
    model::{ApplicationReq, Pagination},
    service::{ApplicationService, ApplicationServiceTrait},
};

pub struct ApplicationController;

impl ApplicationController {
    pub fn new() -> Self {
        ApplicationController {}
    }

    pub fn routes(&self, pg_pool: Arc<PgPool>) -> Router {
        let application_service: Arc<dyn ApplicationServiceTrait + Send + Sync> = Arc::new(ApplicationService::new(Arc::clone(&pg_pool)));

        Router::new()
            .route(
                "/application",
                get(ApplicationController::find_all).post(ApplicationController::save),
            )
            .route("/application/:id", get(ApplicationController::find_by_id))
            .with_state(Arc::clone(&application_service))
    }

    async fn find_all(
        Query(pagination): Query<Pagination>,
        State(application_service): State<Arc<dyn ApplicationServiceTrait + Send + Sync>>,
    ) -> Result<impl IntoResponse, ApiError> {
        let response = application_service.find_all(pagination).await?;
        Ok((StatusCode::OK, Json(response)))
    }

    async fn find_by_id(
        Path(id): Path<i64>,
        State(application_service): State<Arc<dyn ApplicationServiceTrait + Send + Sync>>,
    ) -> Result<impl IntoResponse, ApiError> {
        let response = application_service.find_by_id(id).await?;
        Ok((StatusCode::OK, Json(response)))
    }

    async fn save(
        State(application_service): State<Arc<dyn ApplicationServiceTrait + Send + Sync>>,
        extract::Json(entity): extract::Json<ApplicationReq>,
    ) -> Result<impl IntoResponse, ApiError> {
        let response = application_service.save(entity).await?;
        Ok((StatusCode::OK, Json(response)))
    }
}
