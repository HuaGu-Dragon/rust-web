use std::sync::LazyLock;

use anyhow::Context;
use config::Config;
use serde::Deserialize;

use server::ServerConfig;

use database::DataBaseConfig;

use auth::JwtConfig;

use crate::config::ssl::SslConfig;

mod auth;
pub mod database;
pub mod server;
pub mod ssl;

static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to load configuration"));

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    auth: JwtConfig,
    ssl: SslConfig,
    server: ServerConfig,
    database: DataBaseConfig,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        Config::builder()
            .add_source(
                config::File::with_name("application")
                    .format(config::FileFormat::Toml)
                    .required(true),
            )
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()
            .context("Build the configuration")?
            .try_deserialize()
            .context("Deserialize the configuration")
    }

    pub fn auth(&self) -> &JwtConfig {
        &self.auth
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn ssl(&self) -> &SslConfig {
        &self.ssl
    }

    pub fn database(&self) -> &DataBaseConfig {
        &self.database
    }
}

pub fn get() -> &'static AppConfig {
    &CONFIG
}
