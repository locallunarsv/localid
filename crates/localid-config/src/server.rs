//! Server configuration.

/// Server runtime configuration.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// OIDC issuer URL.
    pub issuer: String,

    /// Signing key file path.
    pub signing_key_path: String,
}

impl ServerConfig {
    /// Creates server configuration.
    #[must_use]
    pub fn new(issuer: impl Into<String>) -> Self {
        Self {
            issuer: issuer.into(),
            signing_key_path: "~/.local/share/localid/keys/signing-key.pem".to_string(),
        }
    }
}
