use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStoryPoint{
    pub story_name: String,
    pub story_points: i32,
}
