use crate::{
    config::AppConfig,
    dtos::issue_payload::{
        IncomingFields, JiraFields, JiraIssueType, JiraIssueUpdate, JiraPayload, JiraProject,
    },
    error::AppError,
};
use axum::Json;
use reqwest::Client;
use serde_json::json;
 
pub async fn create_jira_subtasks(
    config: &AppConfig,
    payload: Vec<IncomingFields>,
) -> Result<Json<serde_json::Value>, AppError> {
    let client = Client::new();
    let payload = convert_to_jira_payload(payload);
    let url = format!("{}/rest/api/2/issue/bulk", config.base_url);
 
    let res: serde_json::Value = client
        .post(url)
        .basic_auth(&config.email, Some(&config.api_token))
        .json(&payload)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
 
    println!("Jira Response: {:#?}", res);
 
 
    let mut created_tasks: Vec<serde_json::Value> = Vec::new();
    let mut error_messages: Vec<String> = Vec::new();
 
    if let Some(issues) = res.get("issues").and_then(|v| v.as_array()) {
        for issue in issues {
            let id = issue.get("id").and_then(|v| v.as_str()).unwrap_or_default();
            let key = issue
                .get("key")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let link = issue
                .get("self")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
 
            created_tasks.push(json!({
                "id": id,
                "key": key,
                "link": link
            }));
        }
    }
 
    if let Some(arr) = res.get("errors").and_then(|v| v.as_array()) {
        let is_unauthorized = arr
            .iter()
            .any(|item| item.get("status").and_then(|v| v.as_i64()) == Some(401));
 
        if is_unauthorized {
            if let Some(first) = arr.first() {
                if let Some(msg) = first
                    .get("elementErrors")
                    .and_then(|e| e.get("errorMessages"))
                    .and_then(|msgs| msgs.as_array())
                    .and_then(|a| a.first())
                    .and_then(|v| v.as_str())
                {
                    error_messages.push(msg.to_string());
                }
            }
        }
 
        let has_parent_error = arr.iter().any(|item| {
            item.get("elementErrors")
                .and_then(|e| e.get("errors"))
                .and_then(|err| err.get("parent"))
                .is_some()
        });
 
        if has_parent_error {
            let failed_indexes: Vec<usize> = arr
                .iter()
                .filter_map(|item| item.get("failedElementNumber")?.as_u64())
                .map(|v| v as usize)
                .collect();
 
            let mut set = std::collections::HashSet::new();
 
            for idx in failed_indexes {
                if let Some(issue) = payload.issue_updates.get(idx) {
                    set.insert(issue.fields.parent.key.clone());
                }
            }
 
            let mut unique: Vec<String> = set.into_iter().collect();
            unique.sort();
 
            for key in unique {
                error_messages.push(format!("Subtasks for story {} were not created", key));
            }
        }
    }
 
    if !created_tasks.is_empty() && error_messages.is_empty() {
        return Ok(Json(json!({
            "created_tasks": created_tasks,
            "error_messages": null,
            "status": "ok"
        })));
    }
 
    if created_tasks.is_empty() && !error_messages.is_empty() {
        let error_json = json!({
            "created_tasks": serde_json::Value::Null,
            "error_messages": error_messages,
            "status": "error"
        });
 
        return Err(AppError(anyhow::anyhow!(error_json)));
    }
 
    let partial_tasks = json!({
        "created_tasks": created_tasks,
        "error_messages": error_messages,
        "status": "error"
    });
    return Err(AppError(anyhow::anyhow!(partial_tasks)));
}
 
pub fn convert_to_jira_payload(items: Vec<IncomingFields>) -> JiraPayload {
    let project_key = std::env::var("JIRA_PROJECT_KEY").unwrap();
    let issue_type_id = std::env::var("JIRA_ISSUE_TYPE_ID").unwrap();
 
    let mut updates: Vec<JiraIssueUpdate> = items
        .into_iter()
        .map(|item| JiraIssueUpdate {
            fields: JiraFields {
                project: JiraProject {
                    key: project_key.clone(),
                },
                parent: JiraProject {
                    key: item.parent.key,
                },
                summary: item.summary,
                issuetype: JiraIssueType {
                    id: issue_type_id.clone(),
                },
            },
        })
        .collect();
    updates.sort_by(|a, b| a.fields.parent.key.cmp(&b.fields.parent.key));
    JiraPayload {
        issue_updates: updates,
    }
}
 