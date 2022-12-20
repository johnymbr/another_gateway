use std::sync::Arc;

use axum::{
    async_trait,
    http::{Request, Response},
};
use hyper::Body;

use crate::exception::{ApiError, APP_ERR_ID_IS_REQUIRED};

use super::ApplicationServiceTrait;

#[async_trait]
pub trait ForwardServiceTrait {
    async fn handle(&self, mut req: Request<Body>) -> Result<Response<Body>, ApiError>;
}

pub struct ForwardService {
    application_service: Arc<dyn ApplicationServiceTrait + Send + Sync>,
}

#[async_trait]
impl ForwardServiceTrait for ForwardService {
    async fn handle(&self, mut req: Request<Body>) ->  Result<Response<Body>, ApiError> {
        if let Some(applicationId) =req.headers().get("X-Application-Id") {
            Ok(Response::new(Body::empty()))
        } else {
            Err(ApiError::new(APP_ERR_ID_IS_REQUIRED))
        }
    }
}