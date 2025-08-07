use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DataBaseConfig {
    pub r#type: DatabaseType,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
    pub schema: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum DatabaseType {
    Postgres,
    Sqlite,
}

impl DataBaseConfig {
    pub fn get_type(&self) -> &DatabaseType {
        &self.r#type
    }

    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("127.0.0.1")
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
