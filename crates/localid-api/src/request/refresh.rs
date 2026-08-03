use serde::Deserialize;

/// Refresh token request payload.
#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    refresh_token: String,
}

impl RefreshRequest {
    /// Returns refresh token secret.
    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }
}
