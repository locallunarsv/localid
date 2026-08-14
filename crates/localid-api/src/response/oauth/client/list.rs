use localid_oauth_client::OAuthClient;

/// Response body for OAuth client list.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ListOAuthClientsResponseBody {
    clients: Vec<OAuthClientResponse>,
}

impl ListOAuthClientsResponseBody {
    /// Creates list response.
    #[must_use]
    pub fn new(clients: Vec<OAuthClientResponse>) -> Self {
        Self { clients }
    }
}

/// OAuth client public response.
#[derive(Debug, Clone, serde::Serialize)]
pub struct OAuthClientResponse {
    client_id: String,
    name: String,
    redirect_uris: Vec<String>,
    state: String,
}

impl From<OAuthClient> for OAuthClientResponse {
    fn from(client: OAuthClient) -> Self {
        Self {
            client_id: client.client_id().to_string(),
            name: client.name().to_string(),
            redirect_uris: client.redirect_uris().to_vec(),
            state: format!("{:?}", client.state()).to_lowercase(),
        }
    }
}
