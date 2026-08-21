//! Server configuration.

use serde::Deserialize;

/// Server runtime configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    /// Server bind host.
    #[serde(default = "default_host")]
    pub host: String,

    /// Server bind port.
    #[serde(default = "default_port")]
    pub port: u16,

    /// OIDC issuer URL.
    pub issuer: String,

    /// Signing key file path.
    #[serde(default = "default_signing_key_path")]
    pub signing_key_path: String,
}

impl ServerConfig {
    /// Creates server configuration.
    #[must_use]
    pub fn new(issuer: impl Into<String>) -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            issuer: issuer.into(),
            signing_key_path: default_signing_key_path(),
        }
    }
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

const fn default_port() -> u16 {
    8080
}

fn default_signing_key_path() -> String {
    "~/.local/share/localid/keys/signing-key.pem".to_string()
}
