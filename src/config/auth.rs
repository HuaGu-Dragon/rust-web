use jsonwebtoken::Algorithm;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JwtConfig {
    pub secret: Option<String>,
    pub algorithm: Option<Algorithm>,
    pub expiration: Option<u64>,
}

impl JwtConfig {
    pub fn secret(&self) -> &str {
        self.secret.as_deref().unwrap_or("default_secret")
    }

    pub fn algorithm(&self) -> Algorithm {
        self.algorithm.unwrap_or_default()
    }

    pub fn expiration(&self) -> u64 {
        self.expiration.unwrap_or(3600)
    }
}
