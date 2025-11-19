use std::sync::Arc;

use axum::{Router, routing::post};
use crate::{config::{self, AppConfig}, handlers::issues_handler::create_subtasks_handler};

pub fn create_routes(config: Arc<AppConfig>) -> Router {
    Router::new()
        .route("/create-subtasks", post(create_subtasks_handler))
        .with_state(config)
}