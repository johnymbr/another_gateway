use crate::config::Db;
use crate::rest::ApplicationController;
use std::{net::SocketAddr, sync::Arc, str::FromStr};

use axum::{Json, Router};
use common::{exception, model};
use hyper::StatusCode;
use opentelemetry_otlp::WithExportConfig;
use serde_json::{json, Value};
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;

pub mod config;
pub mod repository;
pub mod rest;
pub mod service;

#[tokio::main]
async fn main() {
    // load api.env file
    dotenvy::from_filename("api.env").ok();

    // log file appender
    // let file_appender = tracing_appender::rolling::daily(
    //     std::env::var("LOG_PATH").unwrap_or_else(|_| ".".into()),
    //     "ag-api.log",
    // );
    // let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(create_otlp_tracer());

    // initializing tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .with(telemetry_layer)
        // .with(tracing_subscriber::fmt::layer().with_ansi(false))
        // .with(
        //     tracing_subscriber::fmt::Layer::default()
        //         .with_writer(non_blocking)
        //         .with_ansi(false),
        // )
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

fn create_otlp_tracer() -> opentelemetry::sdk::trace::Tracer {
    let protocol = std::env::var("OTEL_EXPORTER_OTLP_PROTOCOL").unwrap_or("grpc".to_string());

    let mut tracer = opentelemetry_otlp::new_pipeline().tracing();
    let headers = parse_otlp_headers_from_env();

    match protocol.as_str() {
        "grpc" => {
            let mut exporter = opentelemetry_otlp::new_exporter()
                .tonic()
                .with_metadata(metadata_from_headers(headers))
                .with_env();

            if let Ok(endpoint) = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT") {
                if endpoint.starts_with("https") {
                    exporter = exporter.with_tls_config(Default::default());
                }
            }
            tracer = tracer.with_exporter(exporter);
        }
        "http/protobuf" => {
            let exporter = opentelemetry_otlp::new_exporter()
                .http()
                .with_headers(headers.into_iter().collect())
                .with_env();
            tracer = tracer.with_exporter(exporter);
        }
        p => panic!("Unsupported protocol {}", p),
    };

    tracer.install_batch(opentelemetry::runtime::Tokio).unwrap()
}

fn metadata_from_headers(headers: Vec<(String, String)>) -> tonic::metadata::MetadataMap {
    use tonic::metadata;

    let mut metadata = metadata::MetadataMap::new();
    headers.into_iter().for_each(|(name, value)| {
        let value = value
            .parse::<metadata::MetadataValue<metadata::Ascii>>()
            .expect("Header value invalid");
        metadata.insert(metadata::MetadataKey::from_str(&name).unwrap(), value);
    });
    metadata
}

fn parse_otlp_headers_from_env() -> Vec<(String, String)> {
    let mut headers = Vec::new();

    if let Ok(hdrs) = std::env::var("OTEL_EXPORTER_OTLP_HEADERS") {
        hdrs.split(',')
            .map(|header| {
                header
                    .split_once('=')
                    .expect("Header should contain '=' character")
            })
            .for_each(|(name, value)| headers.push((name.to_owned(), value.to_owned())));
    }
    headers
}
