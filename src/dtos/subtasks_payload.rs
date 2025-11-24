use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IncomingParentPayload {
    pub key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IncomingFields {
    pub parent: IncomingParentPayload,
    pub summary: String,
}

#[derive(Debug, Serialize)]
pub struct JiraProject {
    pub key: String,
}

#[derive(Debug, Serialize)]
pub struct JiraIssueType {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct JiraFields {
    pub project: JiraProject,
    pub parent: JiraProject,
    pub summary: String,
    pub issuetype: JiraIssueType,
}

#[derive(Debug, Serialize)]
pub struct JiraIssueUpdate {
    pub fields: JiraFields,
}

#[derive(Debug, Serialize)]
pub struct JiraPayload {
    #[serde(rename = "issueUpdates")]
    pub issue_updates: Vec<JiraIssueUpdate>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TaskInput {
    pub subtask: String,
    pub r#type: String,
}
