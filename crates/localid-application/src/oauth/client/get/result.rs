use localid_oauth_client::OAuthClient;

/// Result returned from OAuth client lookup.
#[derive(Debug, Clone)]
pub struct GetOAuthClientResult {
    client: OAuthClient,
}

impl GetOAuthClientResult {
    /// Creates a result.
    #[must_use]
    pub const fn new(client: OAuthClient) -> Self {
        Self { client }
    }

    /// Returns OAuth client.
    #[must_use]
    pub const fn client(&self) -> &OAuthClient {
        &self.client
    }
}
