use std::sync::Arc;

use crate::{config::AppConfig, handlers::story_handler::update_story_points_handler};
use axum::{Router, routing::post};

pub fn story_routes(config: Arc<AppConfig>) -> Router {
    Router::new()
        .route("/update-story-points", post(update_story_points_handler))
        .with_state(config)
}
