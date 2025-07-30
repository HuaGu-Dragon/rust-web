use axum::{Router, extract::State, response::IntoResponse, routing};
use entity::prelude::*;
use sea_orm::prelude::*;

use crate::app::AppState;

mod api;
mod app;
mod config;
mod database;
mod entity;
mod logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new()
        .route("/", routing::get(|| async { "Hello, World!" }))
        .route("/users", routing::get(users));

    app::run(router).await
}

async fn users(State(AppState { db }): State<AppState>) -> impl IntoResponse {
    axum::Json(SysUser::find().all(&db).await.unwrap())
}
