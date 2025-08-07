use axum::{Router, routing};
use tower_http::{auth::AsyncRequireAuthorizationLayer, compression::CompressionLayer};
use tracing::warn;

use crate::{
    app::{
        AppState,
        error::{ApiError, ApiResult},
        middleware::AuthLayer,
    },
    web::{index_handler, static_assets_handler},
};

mod auth;
mod user;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .nest(
                    "/users",
                    user::create_router().layer(AsyncRequireAuthorizationLayer::new(AuthLayer)),
                )
                .nest("/auth", auth::create_router())
                .fallback(async || -> ApiResult<()> {
                    warn!("Not Found");
                    Err(ApiError::NotFound)
                }),
        )
        .nest(
            "/static",
            Router::new().route(
                "/{*file}",
                routing::get(static_assets_handler).route_layer(CompressionLayer::new()),
            ),
        )
        .method_not_allowed_fallback(async || -> ApiResult<()> {
            warn!("Method Not Allowed");
            Err(ApiError::MethodNotAllowed)
        })
        .fallback(index_handler)
}
