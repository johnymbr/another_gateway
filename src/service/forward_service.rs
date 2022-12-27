use std::sync::Arc;

use axum::{
    async_trait,
    http::{Request, Response, uri::Uri},
};
use hyper::{Body, Client};
use sqlx::PgPool;

use crate::exception::{ApiError, FORWARD_ERR_PATH_IS_REQUIRED};

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
    async fn handle(&self, mut req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let path = req.uri().path();
        tracing::info!("{}", path);

        let mut path_segment: Vec<&str> = path.split("/").collect();
        path_segment = path_segment.into_iter().filter(|segment| !segment.trim().is_empty() || segment.eq(&"/")).collect::<Vec<&str>>();
        if path_segment.is_empty() {
            return Err(ApiError::new(FORWARD_ERR_PATH_IS_REQUIRED));
        }

        if let Some(path_application) = path_segment.first() {
            tracing::info!("{:?}", path_application);

            let application = self.application_service.find_by_path((String::from("/") + path_application).as_str()).await?;
            tracing::info!("{:?}", application.path);

            // TODO: create hyper Client, change uri of request and send new request.
            let mut new_path_and_query = path_segment.join("/");
            if let Some(query) = req.uri().query() {
                new_path_and_query = new_path_and_query + query;
            }

            let new_uri = format!("{}/{}", application.url_destination, new_path_and_query);

            *req.uri_mut() = Uri::try_from(new_uri).unwrap();

            let client = Client::new();

            client.request(req).await
            // Ok(Response::new(Body::empty()))
        } else {
            Err(ApiError::new(FORWARD_ERR_PATH_IS_REQUIRED))
        }
    }
}
