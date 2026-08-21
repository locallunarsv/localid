//! Database configuration.

use serde::Deserialize;

/// Database runtime configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    /// PostgreSQL connection URL.
    pub url: String,

    /// Maximum number of database connections.
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

impl DatabaseConfig {
    /// Creates database configuration.
    #[must_use]
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            max_connections: default_max_connections(),
        }
    }

    /// Creates database configuration from `LOCALID_DATABASE_URL`.
    ///
    /// # Errors
    ///
    /// Returns [`std::env::VarError`] when `LOCALID_DATABASE_URL`
    /// is not available in the process environment.
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let url = std::env::var("LOCALID_DATABASE_URL")?;

        Ok(Self::new(url))
    }

    /// Returns PostgreSQL connection URL.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns maximum pool connections.
    #[must_use]
    pub const fn max_connections(&self) -> u32 {
        self.max_connections
    }
}

const fn default_max_connections() -> u32 {
    10
}
