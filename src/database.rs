use std::time::Duration;

use anyhow::Context;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config::{self, database::DatabaseType};

pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let config = config::get().database();
    let mut option = match config.get_type() {
        DatabaseType::Postgres => ConnectOptions::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            config.username(),
            config.password(),
            config.host(),
            config.port(),
            config.database(),
        )),
        DatabaseType::Sqlite => {
            ConnectOptions::new(format!("sqlite:{}?mode=rwc", config.database()))
        }
    };
    option
        .max_connections(20)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(300))
        .sqlx_logging(false)
        .set_schema_search_path(config.schema());

    Database::connect(option)
        .await
        .context("Connect to the database")
}
