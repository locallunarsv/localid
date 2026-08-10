use std::str::FromStr;

use localid_oauth_authorization::AuthorizationCodeId;
use serde::Deserialize;

/// OAuth token request payload.
#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    grant_type: Option<String>,
    code_id: Option<String>,
    client_id: String,
    redirect_uri: Option<String>,
    refresh_token: Option<String>,
}

impl TokenRequest {
    /// Returns grant type.
    #[must_use]
    pub fn grant_type(&self) -> &str {
        self.grant_type.as_deref().unwrap_or("authorization_code")
    }

    /// Returns authorization code identifier.
    pub fn code_id(&self) -> Result<AuthorizationCodeId, uuid::Error> {
        AuthorizationCodeId::from_str(self.code_id.as_deref().unwrap_or_default())
    }

    /// Returns OAuth client identifier.
    #[must_use]
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Returns redirect URI.
    #[must_use]
    pub fn redirect_uri(&self) -> Option<&str> {
        self.redirect_uri.as_deref()
    }

    /// Returns refresh token.
    #[must_use]
    pub fn refresh_token(&self) -> Option<&str> {
        self.refresh_token.as_deref()
    }
}
