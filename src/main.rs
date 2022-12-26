extern crate derive_more;
extern crate serde;

use crate::config::Db;
use crate::rest::{ApplicationController, ForwardController};

use axum::routing::any;
use axum::{Json, Router};
use dotenv::dotenv;
use hyper::StatusCode;
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod config;
pub mod exception;
pub mod model;
pub mod repository;
pub mod rest;
pub mod service;

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
    let forward_controller = ForwardController::new(Arc::clone(&pg_pool));

    let app = Router::new()
        .nest(
            "/api",
            ApplicationController::new()
                .routes(Arc::clone(&pg_pool))
                .fallback(api_fallback),
        )
        .route(
            "/*path",
            any(ForwardController::handle)
                .with_state(Arc::clone(&forward_controller.forward_service)),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    // axum_server::bind_rustls(addr, Rustls::config().await)
}
