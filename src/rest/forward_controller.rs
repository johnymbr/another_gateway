use std::{convert::Infallible, sync::Arc};

use axum::{
    extract::State,
    http::{Request, Response},
};
use hyper::{Body, StatusCode};
use sqlx::PgPool;

use crate::{
    exception::ApiError,
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

    // #[debug_handler]
    pub async fn handle(
        State(forward_service): State<Arc<dyn ForwardServiceTrait + Send + Sync>>,
        req: Request<Body>,
    ) -> Result<Response<Body>, ApiError> {
        let result = forward_service.handle(req).await?;
        Ok(result)
    }

    pub async fn handle_old(
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
