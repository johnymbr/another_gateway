use axum::{
    extract::State,
    http::{uri::Uri, Request, Response},
    routing::get,
    Router,
};
use hyper::{client::HttpConnector, Body, Client};
use std::net::SocketAddr;

async fn root() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    // initializing tracing
    tracing_subscriber::fmt().init();

    let client = Client::new();

    let app = Router::new().route("/", get(root)).with_state(client);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("Hello, world!");
}
