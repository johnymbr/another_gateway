use std::{sync::Arc, collections::VecDeque};

use axum::{
    async_trait,
    http::{Request, Response, uri::Uri},
};
use hyper::{Body, Client, header::HeaderName, client::HttpConnector, StatusCode};
use hyper_tls::HttpsConnector;
use sqlx::PgPool;

use crate::exception::{ApiError, FORWARD_ERR_PATH_IS_REQUIRED};

use super::{ApplicationService, ApplicationServiceTrait};

#[async_trait]
pub trait ForwardServiceTrait {
    async fn handle(&self, mut req: Request<Body>) -> Result<Response<Body>, ApiError>;
}

pub struct ForwardService {
    application_service: Arc<dyn ApplicationServiceTrait + Send + Sync>,
    client: Arc<Client<HttpsConnector<HttpConnector>>>,
}

impl ForwardService {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        let client = Client::builder().build(HttpsConnector::new());
        ForwardService {
            application_service: Arc::new(ApplicationService::new(Arc::clone(&pg_pool))),
            client: Arc::new(client)
        }
    }
}

#[async_trait]
impl ForwardServiceTrait for ForwardService {
    async fn handle(&self, mut req: Request<Body>) -> Result<Response<Body>, ApiError> {
        let path = req.uri().path();
        tracing::info!("{}", path);

        let mut path_segment = path.split('/').collect::<VecDeque<&str>>();
        path_segment = path_segment.into_iter().filter(|segment| !segment.trim().is_empty() || segment.eq(&"/")).collect::<VecDeque<&str>>();
        if path_segment.is_empty() {
            return Err(ApiError::new(FORWARD_ERR_PATH_IS_REQUIRED));
        }

        if let Some(path_application) = path_segment.pop_front() {
            tracing::info!("{:?}", path_application);

            // let application = self.application_service.find_by_path((String::from("/") + path_application).as_str()).await?;
            // tracing::info!("{:?}", application.path);

            // let mut new_path_and_query = Vec::from(path_segment).join("/");
            // if let Some(query) = req.uri().query() {
            //     new_path_and_query += query;
            // }

            // let new_uri = format!("{}/{}", application.url_destination, new_path_and_query);

            // *req.uri_mut() = Uri::try_from(new_uri).unwrap();
            // req.headers_mut().remove(HeaderName::from_static("host"));

            // tracing::info!("{:?}", req);

            // let response = self.client.request(req).await.map_err(|e| {
            //     tracing::error!("{:?}", e);
            //     e
            // })?;

            // tracing::info!("{:?}", response);

            Ok(Response::new("Hello world".into()))
        } else {
            Err(ApiError::new(FORWARD_ERR_PATH_IS_REQUIRED))
        }
    }
}
