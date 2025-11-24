use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub base_url: String,
    pub api_token: String,
    pub email: String,
    pub story_points_custom_field_id: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        Ok(Self {
            base_url: std::env::var("JIRA_BASE_URL")?,
            api_token: std::env::var("JIRA_API_TOKEN")?,
            email: std::env::var("JIRA_EMAIL")?,
            story_points_custom_field_id: std::env::var("JIRA_STORYPOINT_CF")?.parse()?,
        })
    }
}