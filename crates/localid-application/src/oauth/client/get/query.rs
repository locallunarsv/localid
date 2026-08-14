/// Query to retrieve OAuth client.
#[derive(Debug, Clone, Copy)]
pub struct GetOAuthClientQuery {
    client_id: localid_oauth_client::OAuthClientId,
}

impl GetOAuthClientQuery {
    /// Creates a new query.
    #[must_use]
    pub const fn new(client_id: localid_oauth_client::OAuthClientId) -> Self {
        Self { client_id }
    }

    /// Returns OAuth client identifier.
    #[must_use]
    pub const fn client_id(&self) -> localid_oauth_client::OAuthClientId {
        self.client_id
    }
}
