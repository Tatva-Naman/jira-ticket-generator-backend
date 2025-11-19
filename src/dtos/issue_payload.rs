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

// #[derive(Debug, Deserialize, Serialize)]
// pub struct IssuePayload {
//     pub key: IssueUpdates,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct IssueUpdates{
//     pub fields: Fields,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Fields{
//     pub parent: IncomingParentPayload,
//     pub project: IncomingProject,
//     pub summary: String,
//     pub issuetype: IncomingIssueType,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct IncomingProject {
//     pub key: String,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct IncomingIssueType {
//     pub name: String,
// }
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
    pub issueUpdates: Vec<JiraIssueUpdate>,
}

