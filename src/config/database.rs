use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DataBaseConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
    pub schema: Option<String>,
}

impl DataBaseConfig {
    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("localhost")
    }

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(5432)
    }

    pub fn username(&self) -> &str {
        self.username.as_deref().unwrap_or("postgres")
    }

    pub fn password(&self) -> &str {
        self.password.as_deref().unwrap_or("postgres")
    }

    pub fn database(&self) -> &str {
        self.database.as_deref().unwrap_or("postgres")
    }

    pub fn schema(&self) -> &str {
        self.schema.as_deref().unwrap_or("public")
    }
}
