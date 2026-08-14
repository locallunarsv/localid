/// Response returned after OAuth client creation.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CreateOAuthClientResponseBody {
    client_id: String,
    client_secret: String,
}

impl CreateOAuthClientResponseBody {
    /// Creates OAuth client creation response.
    #[must_use]
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }
}
