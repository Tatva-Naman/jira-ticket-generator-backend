use crate::config::AppConfig;
use crate::dtos::subtasks_payload::TaskInput;
use crate::error::AppError;
use crate::services::subtasks_service::search_jira_tasks;
use crate::{dtos::subtasks_payload::IncomingFields, services::subtasks_service::create_jira_subtasks};
use axum::{Json, extract::State};
use std::sync::Arc;

pub async fn create_subtasks_handler(
    State(state): State<Arc<AppConfig>>,
    Json(payload): Json<Vec<IncomingFields>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let response: Result<Json<serde_json::Value>, AppError> =
        create_jira_subtasks(&state, payload).await;
    response
}

pub async fn search_tasks_handler(
    State(state): State<Arc<AppConfig>>,
    Json(payload): Json<Vec<TaskInput>>,
) -> Result<Json<serde_json::Value>, AppError>{
    let response: Result<Json<serde_json::Value>, AppError> =
        search_jira_tasks(&state, payload).await;
    response
}