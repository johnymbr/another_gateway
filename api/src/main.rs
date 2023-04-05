use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


pub mod config;

#[tokio::main]
async fn main() {
    // load api.env file
    dotenv::from_filename("api.env").ok();

    // log file appender
    let file_appender = tracing_appender::rolling::daily(".", "ag-api.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // initializing tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "ag_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::fmt::Layer::default().with_writer(non_blocking))
        .init();
    println!("Hello, world!");
}
