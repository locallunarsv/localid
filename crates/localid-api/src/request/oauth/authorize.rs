use std::str::FromStr;

use localid_identity::IdentityId;
use serde::Deserialize;

/// OAuth authorization request query.
#[derive(Debug, Deserialize)]
pub struct AuthorizeRequest {
    client_id: String,
    redirect_uri: String,
    scope: Option<String>,
    identity_id: String,
    nonce: Option<String>,
    state: Option<String>,
    response_type: String,
}

impl AuthorizeRequest {
    /// Returns OAuth client identifier.
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Returns identity identifier.
    pub fn identity_id(&self) -> Result<IdentityId, uuid::Error> {
        IdentityId::from_str(&self.identity_id)
    }

    /// Returns redirect URI.
    #[must_use]
    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }

    /// Returns requested scopes.
    #[must_use]
    pub fn scope(&self) -> Vec<String> {
        self.scope
            .as_deref()
            .unwrap_or_default()
            .split_whitespace()
            .map(ToOwned::to_owned)
            .collect()
    }

    /// Returns OpenID Connect nonce.
    #[must_use]
    pub fn nonce(&self) -> Option<&str> {
        self.nonce.as_deref()
    }

    /// Returns OAuth state parameter.
    #[must_use]
    pub fn state(&self) -> Option<&str> {
        self.state.as_deref()
    }

    /// Returns OAuth response type.
    #[must_use]
    pub fn response_type(&self) -> &str {
        &self.response_type
    }
}
