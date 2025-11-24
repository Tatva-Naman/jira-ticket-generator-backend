use std::sync::Arc;

use axum::{extract::State, Json};
use crate::{config::AppConfig, dtos::story_points_payload::UpdateStoryPoint, error::AppError, services::story_service::update_story_points_service};

pub async fn update_story_points_handler(
    State(config): State<Arc<AppConfig>>,
    Json(payload): Json<Vec<UpdateStoryPoint>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let response: Result<Json<serde_json::Value>, AppError> =
    update_story_points_service(&config, payload).await;
    response
}