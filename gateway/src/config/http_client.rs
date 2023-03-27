use hyper::{Client, client::HttpConnector};

pub struct HttpClient;

impl HttpClient {
    pub fn config() -> Client<HttpConnector> {
        Client::new()
    }
}