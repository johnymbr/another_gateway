use std::sync::Arc;

use axum::{
    async_trait,
    http::{Request, Response},
};
use hyper::Body;
use sqlx::PgPool;

use crate::exception::{ApiError, APP_ERR_ID_IS_REQUIRED};

use super::{ApplicationService, ApplicationServiceTrait};

#[async_trait]
pub trait ForwardServiceTrait {
    async fn handle(&self, mut req: Request<Body>) -> Result<Response<Body>, ApiError>;
}

pub struct ForwardService {
    application_service: Arc<dyn ApplicationServiceTrait + Send + Sync>,
}

impl ForwardService {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        ForwardService {
            application_service: Arc::new(ApplicationService::new(Arc::clone(&pg_pool))),
        }
    }
}

#[async_trait]
impl ForwardServiceTrait for ForwardService {
    async fn handle(&self, mut req: Request<Body>) -> Result<Response<Body>, ApiError> {
        let path = req.uri().path();
        tracing::info!("{}", path);

        let path_segment: Vec<&str> = path.split("/").collect();
        if !path_segment.is_empty() {
            tracing::info!("{:?}", path_segment[1]);
        }

        Ok(Response::new(Body::empty()))
    }
}