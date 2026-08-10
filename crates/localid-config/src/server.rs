//! Server configuration.

/// Server runtime configuration.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    issuer: String,
}

impl ServerConfig {
    /// Creates a new server configuration.
    #[must_use]
    pub fn new(issuer: impl Into<String>) -> Self {
        Self {
            issuer: issuer.into(),
        }
    }

    /// Returns issuer URL.
    #[must_use]
    pub fn issuer(&self) -> &str {
        &self.issuer
    }
}
