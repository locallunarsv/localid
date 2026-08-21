//! Database configuration.

use std::env;

/// Database runtime configuration.
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// PostgreSQL connection URL.
    pub url: String,

    /// Maximum number of database connections.
    pub max_connections: u32,
}

impl DatabaseConfig {
    /// Creates database configuration.
    #[must_use]
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            max_connections: 10,
        }
    }

    /// Creates database configuration from `LOCALID_DATABASE_URL`.
    ///
    /// # Errors
    ///
    /// Returns [`std::env::VarError`] when `LOCALID_DATABASE_URL`
    /// is not available in the process environment.
    pub fn from_env() -> Result<Self, env::VarError> {
        let url = env::var("LOCALID_DATABASE_URL")?;

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
