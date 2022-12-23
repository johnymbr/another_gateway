use std::sync::Arc;

use axum::{
    extract::{self, Path, Query, State},
    http::{Request, Response},
    routing::get,
    Json, Router, Extension,
};
use hyper::{StatusCode, Body};
use sqlx::PgPool;

use crate::{
    exception::ApiError,
    model::{ApplicationReq, Pagination},
    service::{ApplicationService, ApplicationServiceTrait, ForwardServiceTrait, ForwardService},
};

pub struct ForwardController {
    forward_service: Arc<dyn ForwardServiceTrait + Send + Sync>,
}

impl ForwardController {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        let forward_service: Arc<dyn ForwardServiceTrait + Send + Sync> = Arc::new(ForwardService::new(Arc::clone(&pg_pool)));
        ForwardController {
            forward_service
        }
    }

    pub async fn handle(mut req: Request<Body>, Extension(forward_service): Extension<Arc<dyn ForwardServiceTrait + Send + Sync>>) ->  Result<Response<Body>, ApiError> {
        let result = forward_service.handle(req).await?;
        Ok(result)
    }
}
