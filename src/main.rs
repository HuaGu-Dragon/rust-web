use axum::{Router, extract::State, response::IntoResponse, routing};
use entity::prelude::*;
use sea_orm::prelude::*;
use tokio::net::TcpListener;
use tracing::info;

mod config;
mod database;
mod entity;
mod logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    let db = database::init().await?;

    let router = Router::new()
        .route("/", routing::get(|| async { "Hello, World!" }))
        .route("/users", routing::get(users))
        .with_state(db);

    let port = config::get().server().port();

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    info!("listening on http://0.0.0.0:{port}");

    axum::serve(listener, router).await?;

    Ok(())
}

async fn users(State(db): State<DatabaseConnection>) -> impl IntoResponse {
    axum::Json(SysUser::find().all(&db).await.unwrap())
}
