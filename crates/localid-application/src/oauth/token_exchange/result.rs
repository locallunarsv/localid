use chrono::{DateTime, Utc};

/// Result returned after successful OAuth token exchange.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenExchangeResult {
    access_token: String,
    refresh_token: String,
    id_token: Option<String>,
    expires_at: DateTime<Utc>,
}

impl TokenExchangeResult {
    /// Creates a token exchange result.
    #[must_use]
    pub fn new(
        access_token: impl Into<String>,
        refresh_token: impl Into<String>,
        id_token: Option<String>,
        expires_at: DateTime<Utc>,
    ) -> Self {
        Self {
            access_token: access_token.into(),
            refresh_token: refresh_token.into(),
            id_token,
            expires_at,
        }
    }

    /// Returns access token.
    #[must_use]
    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    /// Returns refresh token.
    #[must_use]
    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }

    /// Returns ID token when OpenID Connect scope was requested.
    #[must_use]
    pub fn id_token(&self) -> Option<&str> {
        self.id_token.as_deref()
    }

    /// Returns token expiration time.
    #[must_use]
    pub const fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }
}
