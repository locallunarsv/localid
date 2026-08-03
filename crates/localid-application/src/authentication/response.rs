use chrono::{DateTime, Utc};

use localid_authentication::AuthenticateResult;

/// Response returned after successful login.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginResponse {
    access_token: String,
    refresh_token: String,
    expires_at: DateTime<Utc>,
}

impl LoginResponse {
    /// Creates a login response from authentication result.
    #[must_use]
    pub fn from_authentication_result(result: &AuthenticateResult) -> Self {
        Self {
            access_token: result.token().secret().to_owned(),
            refresh_token: result.refresh_token().secret().to_owned(),
            expires_at: result.session().expires_at(),
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

    /// Returns expiration time.
    #[must_use]
    pub const fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }
}
