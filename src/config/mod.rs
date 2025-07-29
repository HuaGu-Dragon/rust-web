use std::sync::LazyLock;

use anyhow::Context;
use config::Config;
use serde::Deserialize;

use server::ServerConfig;

mod server;

static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to load configuration"));

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
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
}

pub fn get() -> &'static AppConfig {
    &CONFIG
}
