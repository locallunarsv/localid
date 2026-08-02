use localid_refresh_token_random::IssuedRefreshToken;
use localid_token_random::IssuedToken;

/// Result returned after refreshing tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefreshResult {
    access_token: IssuedToken,
    refresh_token: IssuedRefreshToken,
}

impl RefreshResult {
    /// Creates a refresh result.
    #[must_use]
    pub const fn new(access_token: IssuedToken, refresh_token: IssuedRefreshToken) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }

    /// Returns the new access token.
    #[must_use]
    pub const fn access_token(&self) -> &IssuedToken {
        &self.access_token
    }

    /// Returns the new refresh token.
    #[must_use]
    pub const fn refresh_token(&self) -> &IssuedRefreshToken {
        &self.refresh_token
    }
}
