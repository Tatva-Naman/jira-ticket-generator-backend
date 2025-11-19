use crate::config::AppConfig;
use crate::error::AppError;
use crate::{dtos::issue_payload::IncomingFields, services::jira_service::create_jira_subtasks};
use axum::{Json, extract::State};
use std::sync::Arc;

pub async fn create_subtasks_handler(
    State(state): State<Arc<AppConfig>>,
    Json(payload): Json<Vec<IncomingFields>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let response: Result<Json<serde_json::Value>, AppError> = create_jira_subtasks(&state, payload).await;
    response
}