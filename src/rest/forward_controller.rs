use std::{sync::Arc, convert::Infallible};

use axum::{
    http::{Request, Response},
};
use hyper::{Body, StatusCode};
use sqlx::PgPool;

use crate::{
    service::{ForwardService, ForwardServiceTrait},
};

pub struct ForwardController {
    pub forward_service: Arc<dyn ForwardServiceTrait + Send + Sync>,
}

impl ForwardController {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        let forward_service: Arc<dyn ForwardServiceTrait + Send + Sync> =
            Arc::new(ForwardService::new(Arc::clone(&pg_pool)));
        ForwardController { forward_service }
    }

    pub async fn handle(
        req: Request<Body>,
        forward_service: Arc<dyn ForwardServiceTrait + Send + Sync>,
    ) -> Result<Response<Body>, Infallible> {
        let result = forward_service.handle(req).await;

        match result {
            Ok(response) => Ok(response),
            Err(api_error) => {
                let json = serde_json::to_string(&api_error).unwrap();
                Ok(Response::builder()
                    .status(StatusCode::from_u16(api_error.status_code).unwrap())
                    .body(Body::from(json))
                    .unwrap())
            }
        }
    }
}
