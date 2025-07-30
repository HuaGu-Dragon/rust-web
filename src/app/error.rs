use axum::{Json, response::IntoResponse};
use serde::Serialize;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,
    #[error("method Not Allowed")]
    MethodNotAllowed,
    #[error("Internal Server Error")]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub error: String,
}

impl ApiError {
    pub fn status_code(&self) -> axum::http::StatusCode {
        match self {
            ApiError::NotFound => axum::http::StatusCode::NOT_FOUND,
            ApiError::Internal(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::MethodNotAllowed => axum::http::StatusCode::METHOD_NOT_ALLOWED,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let body = Json(ErrorResponse {
            code: status.as_u16(),
            error: self.to_string(),
        });

        (status, body).into_response()
    }
}
