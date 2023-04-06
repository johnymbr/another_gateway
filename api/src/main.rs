use crate::config::Db;
use crate::rest::ApplicationController;
use std::{sync::Arc, net::SocketAddr};

use axum::{Router, Json};
use hyper::StatusCode;
use serde_json::{Value, json};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use common::{exception, model};

pub mod config;
pub mod repository;
pub mod rest;
pub mod service;

#[tokio::main]
async fn main() {
    // load api.env file
    dotenvy::from_filename("api.env").ok();

    // log file appender
    let file_appender = tracing_appender::rolling::daily(
        std::env::var("LOG_PATH").unwrap_or_else(|_| ".".into()),
        "ag-api.log",
    );
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // initializing tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer().with_ansi(false))
        .with(
            tracing_subscriber::fmt::Layer::default()
                .with_writer(non_blocking)
                .with_ansi(false),
        )
        .init();

    let pg_pool = Arc::new(Db::config().await);

    let app = Router::new()
        .nest(
            "/application",
            ApplicationController::new()
                .routes(Arc::clone(&pg_pool))
                .fallback(api_fallback),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn api_fallback() -> (StatusCode, Json<Value>) {
    let body = json!({
        "status": 404,
        "message": "Not Found",
    });
    (StatusCode::NOT_FOUND, Json(body))
}