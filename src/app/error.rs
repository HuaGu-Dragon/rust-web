use argon2::password_hash::Error;
use axum::{
    Json,
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    response::{IntoResponse, Response},
};
use axum_extra::typed_header::TypedHeaderRejection;
use axum_valid::ValidRejection;
use serde::Serialize;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,
    #[error("method Not Allowed")]
    MethodNotAllowed,
    #[error("Invalid query parameters: {0}")]
    InvalidQueryParams(#[from] QueryRejection),
    #[error("Invalid path parameters: {0}")]
    InvalidPathParams(#[from] PathRejection),
    #[error("Invalid json body: {0}")]
    InvalidJsonBody(#[from] JsonRejection),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Failed to hash password: {0}")]
    HashPassword(String),
    #[error("JWT Error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("Failed to extract typed header: {0}")]
    TypedHeaderError(#[from] TypedHeaderRejection),
    #[error("Account or Password is incorrect")]
    LoginError,
    #[error("Internal Server Error")]
    Internal(#[from] anyhow::Error),
}

impl From<ValidRejection<ApiError>> for ApiError {
    fn from(value: ValidRejection<ApiError>) -> Self {
        match value {
            axum_valid::ValidationRejection::Valid(e) => Self::ValidationError(e.to_string()),
            axum_valid::ValidationRejection::Inner(e) => e,
        }
    }
}

impl From<Error> for ApiError {
    fn from(value: Error) -> Self {
        Self::HashPassword(value.to_string())
    }
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
            ApiError::HashPassword(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::MethodNotAllowed => axum::http::StatusCode::METHOD_NOT_ALLOWED,
            ApiError::InvalidQueryParams(_)
            | ApiError::InvalidPathParams(_)
            | ApiError::InvalidJsonBody(_) => axum::http::StatusCode::BAD_REQUEST,
            ApiError::ValidationError(_) => axum::http::StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::Internal(e) => {
                tracing::warn!(error = ?e, "Internal server error");
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            ApiError::JwtError(_) | ApiError::TypedHeaderError(_) | ApiError::LoginError => {
                axum::http::StatusCode::UNAUTHORIZED
            }
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

impl From<ApiError> for Response {
    fn from(value: ApiError) -> Self {
        value.into_response()
    }
}
