use axum::Json;
use futures::future::join_all;
use reqwest::Client;
use serde_json::json;

use crate::{config::AppConfig, dtos::story_points_payload::UpdateStoryPoint, error::AppError};

pub async fn update_story_points_service(
    config: &AppConfig,
    payload: Vec<UpdateStoryPoint>,
) -> Result<Json<serde_json::Value>, AppError> {
    let client = Client::new();

    let futures = payload.into_iter().map(|item| {
        let client = client.clone();
        let base_url = std::env::var("JIRA_BASE_URL").unwrap();
        let story_points_custom_field_id = std::env::var("JIRA_STORYPOINT_CF").unwrap();
        let story_name = item.story_name.clone();
        let new_sp = item.story_points;

        async move {
            let url = format!("{}/rest/api/3/issue/{}", base_url, story_name);
            let body = json!({
                "fields": {
                    story_points_custom_field_id: new_sp
                }
            });
            println!("Updating story: {} with body: {}", story_name, body);
            let result = client
                .put(&url)
                .basic_auth(&config.email, Some(&config.api_token))
                .json(&body)
                .send()
                .await;

            match result {
                Ok(resp) if resp.status().is_success() => Ok(story_name),
                _ => {
                    println!("error recieved while updating story points: {:#?}", result);
                    Err(story_name)
                }
            }
        }
    });

    let results = join_all(futures).await;

    let mut updated_stories = Vec::new();
    let mut failed_stories = Vec::new();

    for res in results {
        match res {
            Ok(story_name) => updated_stories.push(story_name),
            Err(story_name) => failed_stories.push(story_name),
        }
    }

    let response = if failed_stories.is_empty() {
        json!({
            "message": "All story points updated successfully",
            "status": "success",
            "updated_stories": updated_stories,
            "failed_stories": null
        })
    } else if updated_stories.is_empty() {
        json!({
            "message": "All story points update failed",
            "status": "error",
            "updated_stories": null,
            "failed_stories": failed_stories
        })
    } else {
        json!({
            "message": "Some story points updated successfully",
            "status": "error",
            "updated_stories": updated_stories,
            "failed_stories": failed_stories
        })
    };

    if failed_stories.is_empty() {
        Ok(Json(response))
    } else {
        Err(AppError(anyhow::anyhow!(response)))
    }
}
