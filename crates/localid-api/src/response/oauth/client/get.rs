use super::OAuthClientResponse;
use localid_oauth_client::OAuthClient;

/// Response body for OAuth client lookup.
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetOAuthClientResponseBody {
    client: OAuthClientResponse,
}

impl GetOAuthClientResponseBody {
    /// Creates get response.
    #[must_use]
    pub fn new(client: OAuthClientResponse) -> Self {
        Self { client }
    }
}

impl From<&OAuthClient> for GetOAuthClientResponseBody {
    fn from(client: &OAuthClient) -> Self {
        Self::new(OAuthClientResponse::from(client.clone()))
    }
}
