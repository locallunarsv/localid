/// Request to verify an access token.
#[derive(Debug)]
pub struct VerifyTokenQuery {
    token: String,
}

impl VerifyTokenQuery {
    /// Creates a token verification query.
    #[must_use]
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }

    /// Returns access token.
    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }
}
