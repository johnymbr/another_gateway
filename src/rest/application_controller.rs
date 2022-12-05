use axum::{routing::get, Router};

pub struct ApplicationController;

impl ApplicationController {
    pub fn route() -> Router {
        Router::new().route("/application", get(ApplicationController::find_application))
    }

    async fn find_application() -> &'static str {
        "Hello, world!"
    }
}
