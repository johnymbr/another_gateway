use std::path::PathBuf;

use axum_server::tls_rustls::RustlsConfig;

pub struct Rustls;

impl Rustls {
    pub async fn config() -> RustlsConfig {
        RustlsConfig::from_pem_file(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join(std::env::var("CERTS_PATH").unwrap())
                .join(std::env::var("CERT_FILE").unwrap()),
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join(std::env::var("CERTS_PATH").unwrap())
                .join(std::env::var("KEY_FILE").unwrap()),
        )
        .await
        .unwrap()
    }
}
