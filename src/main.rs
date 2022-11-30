use axum::{
    extract::State,
    http::{uri::Uri, Request, Response},
    routing::get,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use hyper::{client::HttpConnector, Body, Client};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{net::SocketAddr, path::PathBuf};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


async fn root() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    // initializing tracing
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "another_gateway=debug".into())
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("another_gateway_cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("another_gateway_key.pem")
    )
    .await
    .unwrap();

    let client = Client::new();

    let app = Router::new().route("/", get(root)).with_state(client);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();

}
