use std::sync::Arc;

use axum::{
    async_trait,
    http::{Request, Response},
};
use hyper::Body;

use super::ApplicationServiceTrait;

#[async_trait]
pub trait ForwardServiceTrait {
    async fn handle(&self, mut req: Request<Body>) -> Response<Body>;
}

pub struct ForwardService {
    application_service: Arc<dyn ApplicationServiceTrait + Send + Sync>,
}
