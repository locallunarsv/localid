use serde::Deserialize;

/// Request payload for token verification.
#[derive(Debug, Deserialize)]
pub struct VerifyTokenRequest {
    token: String,
}

impl VerifyTokenRequest {
    /// Returns token value.
    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }
}
