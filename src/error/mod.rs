use axum::{response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub struct AppError(pub anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!({
            "status": "error",
            "message": self.0.to_string(),
        }));
        (axum::http::StatusCode::BAD_REQUEST, body).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>
{
    fn from(err: E) -> Self {
        AppError(err.into())
    }
}
