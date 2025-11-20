use axum::{Json, response::IntoResponse};
use serde_json::{json, Value};
 
#[derive(Debug)]
pub struct AppError(pub anyhow::Error);
 
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let err_string = self.0.to_string();
 
        // Attempt to parse the error string as JSON
        let parsed: Result<Value, _> = serde_json::from_str(&err_string);
 
        let body = match parsed {
            // If the string is valid JSON, return it as-is
            Ok(json_value) => Json(json_value),
 
            // Otherwise, fallback to old format
            Err(_) => Json(json!({
                "status": "error",
                "message": err_string
            })),
        };
 
        (axum::http::StatusCode::BAD_REQUEST, body).into_response()
    }
}
 
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        AppError(err.into())
    }
}