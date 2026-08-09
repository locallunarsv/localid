use std::str::FromStr;

use localid_oauth_authorization::AuthorizationCodeId;
use serde::Deserialize;

/// OAuth token exchange request payload.
#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    code_id: String,
    client_id: String,
    redirect_uri: String,
}

impl TokenRequest {
    /// Returns authorization code identifier.
    pub fn code_id(&self) -> Result<AuthorizationCodeId, uuid::Error> {
        AuthorizationCodeId::from_str(&self.code_id)
    }

    /// Returns OAuth client identifier.
    #[must_use]
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Returns redirect URI.
    #[must_use]
    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }
}
