use crate::{
    config::AppConfig,
    dtos::subtasks_payload::{
        IncomingFields, JiraFields, JiraIssueType, JiraIssueUpdate, JiraPayload, JiraProject, TaskInput,
    },
    error::AppError,
};
use anyhow::Context;
use axum::Json;
use reqwest::{Client, StatusCode};
use serde_json::{json, Value};
use urlencoding::encode;

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

    let mut created_tasks: Vec<serde_json::Value> = Vec::new();
    let mut error_messages: Vec<String> = Vec::new();

    if let Some(issues) = res.get("issues").and_then(|v| v.as_array()) {
        for issue in issues {
            let id = issue.get("id").and_then(|v| v.as_str()).unwrap_or_default();
            let key = issue
                .get("key")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let link = format!(
                "{}/browse/{}",
                config.base_url,
                key
            );
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

pub async fn search_jira_tasks(
    config: &AppConfig,
    payload: Vec<TaskInput>,
) -> Result<Json<Value>, AppError> 
{
    if payload.is_empty() {
        return Err(anyhow::anyhow!("No tasks provided.").into());
    }

    // Extract only subtask keys
    let parent_keys: Vec<String> = payload
        .iter()
        .map(|t| t.subtask.clone())
        .collect();

    let quoted_keys = parent_keys
        .iter()
        .map(|k| format!("\"{}\"", k))
        .collect::<Vec<_>>()
        .join(", ");

    let jql_query = format!("parent IN ({})", quoted_keys);
    let encoded_jql = encode(&jql_query);
    let url = format!(
        "{}/rest/api/3/search/jql?jql={}&fields=summary",
        config.base_url,
        encoded_jql
    );
    let client = Client::new();

    let response = client
        .get(&url)
        .basic_auth(&config.email, Some(&config.api_token))
        .send()
        .await
        .context("Jira API request failed")?;
    if response.status() != StatusCode::OK {
        let err = response.text().await.unwrap_or("Unknown error".to_string());
        return Err(anyhow::anyhow!(err).into());
    }

    let jira_json: Value = response
        .json()
        .await
        .context("Jira JSON parsing failed")?;

    let actual_summaries = extract_jira_summaries(&jira_json);
    // Final aggregate output
    let mut flat_output = Vec::<Value>::new();

    for item in payload {
        let expected = generate_expected_summaries(&item.subtask, &item.r#type);
    
        let comparison = compare_summaries(expected, actual_summaries.clone());
    
        for entry in comparison {
            flat_output.push(entry);
        }
    }
    
    Ok(Json(json!(flat_output)))    
}

fn generate_expected_summaries(parent: &str, task_type: &str) -> Vec<String> {
    let fe_tasks = vec![
        "Review Requirements",
        "Development",
        "Unit Testing",
    ];

    let be_tasks = vec![
        "Review Requirements",
        "Development",
        "Unit Testing",
    ];

    match task_type {
        "FE" => fe_tasks
            .into_iter()
            .map(|t| format!("({}) FE - {}", parent, t))
            .collect(),

        "BE" => be_tasks
            .into_iter()
            .map(|t| format!("({}) BE - {}", parent, t))
            .collect(),

        "Both" => {
            let mut combined = Vec::new();

            combined.extend(
                fe_tasks.iter().map(|t| format!("({}) FE - {}", parent, t))
            );

            combined.extend(
                be_tasks.iter().map(|t| format!("({}) BE - {}", parent, t))
            );

            combined
        }

        _ => vec![],
    }
}

fn extract_jira_summaries(jira_json: &Value) -> Vec<String> {
    jira_json["issues"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|issue| {
            issue["fields"]["summary"].as_str().map(|s| s.to_string())
        })
        .collect()
}

fn compare_summaries(expected: Vec<String>, actual: Vec<String>) -> Vec<Value> {
    expected
        .into_iter()
        .map(|exp| {
            let exists = actual.contains(&exp);

            json!({
                "summary": exp,
                "exist": exists
            })
        })
        .collect()
}
