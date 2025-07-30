mod api;
mod app;
mod config;
mod database;
mod entity;
mod logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run(api::create_router()).await
}
