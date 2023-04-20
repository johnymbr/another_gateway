use std::sync::Arc;

use axum::{
    extract::{self, Path, Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use hyper::StatusCode;
use sqlx::PgPool;
use tracing::instrument;

use crate::{
    exception::ApiError,
    model::{ApplicationReq, Pagination},
    service::{ApplicationService, ApplicationServiceTrait},
};

pub struct ApplicationController;

impl Default for ApplicationController {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationController {
    pub fn new() -> Self {
        ApplicationController {}
    }

    pub fn routes(&self, pg_pool: Arc<PgPool>) -> Router {
        let application_service: Arc<dyn ApplicationServiceTrait + Send + Sync> =
            Arc::new(ApplicationService::new(Arc::clone(&pg_pool)));

        Router::new()
            .route(
                "/",
                get(ApplicationController::find_all).post(ApplicationController::save),
            )
            .route(
                "/:id",
                get(ApplicationController::find_by_id)
                    .put(ApplicationController::update)
                    .delete(ApplicationController::delete),
            )
            .with_state(Arc::clone(&application_service))
    }

    #[instrument]
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

    async fn update(
        Path(id): Path<i64>,
        State(application_service): State<Arc<dyn ApplicationServiceTrait + Send + Sync>>,
        extract::Json(entity): extract::Json<ApplicationReq>,
    ) -> Result<impl IntoResponse, ApiError> {
        let response = application_service.update(id, entity).await?;
        Ok((StatusCode::OK, Json(response)))
    }

    async fn delete(
        Path(id): Path<i64>,
        State(application_service): State<Arc<dyn ApplicationServiceTrait + Send + Sync>>,
    ) -> Result<impl IntoResponse, ApiError> {
        application_service.delete(id).await?;
        Ok(StatusCode::NO_CONTENT)
    }
}
