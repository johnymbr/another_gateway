use std::sync::Arc;

use axum::{routing::get, Router, extract::{Query, State}, response::IntoResponse, Json};
use hyper::StatusCode;
use sqlx::PgPool;

use crate::{model::Pagination, service::ApplicationService, exception::ApiError};

pub struct ApplicationController;

impl ApplicationController {
    pub fn routes(pg_pool: Arc<PgPool>) -> Router {
        Router::new().route("/application", get(ApplicationController::find_application))
            .with_state(pg_pool)
    }

    async fn find_application(Query(pagination): Query<Pagination>, State(pg_pool): State<Arc<PgPool>>) -> Result<impl IntoResponse, ApiError> {
        let response = ApplicationService::find_application(pagination, &pg_pool).await?;
        Ok((StatusCode::OK, Json(response)))
    }
}
