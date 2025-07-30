use axum::Router;
use tracing::warn;

use crate::app::{
    AppState,
    error::{ApiError, ApiResult},
};

mod user;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/users", user::create_router())
                .fallback(async || -> ApiResult<()> {
                    warn!("Not Found");
                    Err(ApiError::NotFound)
                }),
        )
        .method_not_allowed_fallback(async || -> ApiResult<()> {
            warn!("Method Not Allowed");
            Err(ApiError::MethodNotAllowed)
        })
}
