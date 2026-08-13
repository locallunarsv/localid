/// OAuth client authentication command.
#[derive(Debug, Clone)]
pub struct ClientAuthenticationCommand {
    client_id: String,
    client_secret: String,
}

impl ClientAuthenticationCommand {
    /// Creates a new client authentication command.
    #[must_use]
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }

    /// Returns OAuth client identifier.
    #[must_use]
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Returns OAuth client secret.
    #[must_use]
    pub fn client_secret(&self) -> &str {
        &self.client_secret
    }
}
