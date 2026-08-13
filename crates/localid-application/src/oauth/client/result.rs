/// Result returned after OAuth client creation.
#[derive(Debug, Clone)]
pub struct CreateOAuthClientResult {
    client_id: String,
    client_secret: String,
}

impl CreateOAuthClientResult {
    /// Creates result.
    #[must_use]
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }

    /// Returns public client identifier.
    #[must_use]
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Returns generated client secret.
    #[must_use]
    pub fn client_secret(&self) -> &str {
        &self.client_secret
    }
}
