use axum::{Router, routing};
use tokio::net::TcpListener;
use tracing::info;

mod config;
mod logger;

#[tokio::main]
async fn main() {
    logger::init();

    let router = Router::new().route("/", routing::get(|| async { "Hello, World!" }));

    let port = config::get().server.port();

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

    info!("listening on http://0.0.0.0:{port}");

    axum::serve(listener, router).await.unwrap();
}
