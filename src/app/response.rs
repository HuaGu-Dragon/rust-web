use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub message: String,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            message: "success".into(),
            data,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, Json(self)).into_response()
    }
}
