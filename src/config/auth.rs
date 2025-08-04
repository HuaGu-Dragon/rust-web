use jsonwebtoken::Algorithm;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JwtConfig {
    pub secret: Option<String>,
    pub algorithm: Option<Algorithm>,
}

impl JwtConfig {
    pub fn secret(&self) -> &str {
        self.secret.as_deref().unwrap_or("default_secret")
    }

    pub fn algorithm(&self) -> Algorithm {
        self.algorithm.unwrap_or_default()
    }
}
