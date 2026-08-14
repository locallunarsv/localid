use localid_oauth_client::OAuthClientId;

/// Command to disable OAuth client.
#[derive(Debug, Clone, Copy)]
pub struct DisableOAuthClientCommand {
    client_id: OAuthClientId,
}

impl DisableOAuthClientCommand {
    /// Creates a new disable command.
    #[must_use]
    pub const fn new(client_id: OAuthClientId) -> Self {
        Self { client_id }
    }

    /// Returns OAuth client identifier.
    #[must_use]
    pub const fn client_id(&self) -> OAuthClientId {
        self.client_id
    }
}
