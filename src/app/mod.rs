use axum::Router;
use sea_orm::DatabaseConnection;
use tracing::info;

use crate::{
    app::{error::ApiResult, response::ApiResponse},
    config, database, logger,
};

pub mod error;
pub mod extract;
mod latency;
pub mod params;
pub mod response;
mod server;

pub type ApiReturn<T> = ApiResult<ApiResponse<T>>;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    logger::init();
    info!("Starting application...");

    let db = database::init().await?;
    info!("Database connection established");

    let state = AppState::new(db);
    let server = server::Server::new(config::get().server());

    server.start(router, state).await
}
