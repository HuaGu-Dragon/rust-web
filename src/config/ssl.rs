use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SslConfig {
    pub enable: bool,
    pub cert_path: Option<PathBuf>,
    pub key_path: Option<PathBuf>,
}

impl SslConfig {
    pub fn enable(&self) -> bool {
        self.enable
    }

    pub fn cert_path(&self) -> Option<&PathBuf> {
        self.cert_path.as_ref()
    }

    pub fn key_path(&self) -> Option<&PathBuf> {
        self.key_path.as_ref()
    }
}
