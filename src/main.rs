use crate::config::{Db, HttpClient, Rustls};
use crate::rest::ApplicationController;

use axum::{
    extract::State,
    http::{uri::Uri, Request, Response},
    routing::get,
    Json, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use dotenv::dotenv;
use hyper::{client::HttpConnector, Body, Client, StatusCode};
use serde_json::{json, Value};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use std::{net::SocketAddr, path::PathBuf, time::Duration};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod config;
pub mod exception;
pub mod model;
pub mod rest;
pub mod service;
pub mod repository;

async fn root() -> &'static str {
    "Others routes!!"
}

async fn api_fallback() -> (StatusCode, Json<Value>) {
    let body = json!({
        "status": 404,
        "message": "Not Found",
    });
    (StatusCode::NOT_FOUND, Json(body))
}

#[tokio::main]
async fn main() {
    // load .env file
    dotenv().ok();

    // initializing tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "another_gateway=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pg_pool = Arc::new(Db::config().await);

    let app = Router::new()
        .with_state(Arc::clone(&pg_pool))
        .nest(
            "/api",
            ApplicationController::routes(Arc::clone(&pg_pool)).fallback(api_fallback),
        )
        .route("/", get(root))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);

    axum_server::bind_rustls(addr, Rustls::config().await)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
