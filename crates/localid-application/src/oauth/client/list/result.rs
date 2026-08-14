use localid_oauth_client::OAuthClient;

/// Result returned from listing OAuth clients.
#[derive(Debug, Clone)]
pub struct ListOAuthClientsResult {
    clients: Vec<OAuthClient>,
}

impl ListOAuthClientsResult {
    /// Creates list result.
    #[must_use]
    pub const fn new(clients: Vec<OAuthClient>) -> Self {
        Self { clients }
    }

    /// Returns OAuth clients.
    #[must_use]
    pub fn clients(&self) -> &[OAuthClient] {
        &self.clients
    }
}
