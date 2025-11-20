use std::sync::Arc;
use crate::{config::AppConfig, handlers::issues_handler::{create_subtasks_handler, search_tasks_handler}};
use axum::{routing::post, Router};
 
pub fn create_routes(config: Arc<AppConfig>) -> Router {
    Router::new()
        .route("/create-subtasks", post(create_subtasks_handler))
        .route("/search-subtasks", post(search_tasks_handler))
        .with_state(config)
}