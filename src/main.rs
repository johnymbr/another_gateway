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
use std::{net::SocketAddr, path::PathBuf, time::Duration};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod exception;
pub mod model;
pub mod rest;
pub mod service;

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

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can connect to database");

    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("another_gateway_cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("another_gateway_key.pem"),
    )
    .await
    .unwrap();

    let client = Client::new();

    let app = Router::new()
        .with_state(client)
        .with_state(pool)
        .nest(
            "/api",
            ApplicationController::route().fallback(api_fallback),
        )
        .route("/", get(root))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
