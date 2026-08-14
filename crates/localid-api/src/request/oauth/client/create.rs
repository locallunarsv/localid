use serde::Deserialize;

/// OAuth client creation request payload.
#[derive(Debug, Deserialize)]
pub struct CreateOAuthClientRequest {
    name: String,
    redirect_uris: Vec<String>,
}

impl CreateOAuthClientRequest {
    /// Returns OAuth client name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns registered redirect URIs.
    #[must_use]
    pub fn redirect_uris(&self) -> &[String] {
        &self.redirect_uris
    }
}
