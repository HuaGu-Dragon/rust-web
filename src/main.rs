use axum::{Router, routing};
use tokio::net::TcpListener;
use tracing::info;

mod config;
mod database;
mod logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    database::init().await?;

    let router = Router::new().route("/", routing::get(|| async { "Hello, World!" }));

    let port = config::get().server().port();

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    info!("listening on http://0.0.0.0:{port}");

    axum::serve(listener, router).await?;

    Ok(())
}
