use serde::Deserialize;

/// OAuth token request payload.
#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    grant_type: Option<String>,
    code: Option<String>,
    client_id: String,
    redirect_uri: Option<String>,
    refresh_token: Option<String>,
    code_verifier: Option<String>,
    client_secret: Option<String>,
}

impl TokenRequest {
    /// Returns grant type.
    #[must_use]
    pub fn grant_type(&self) -> &str {
        self.grant_type.as_deref().unwrap_or("authorization_code")
    }

    /// Returns authorization code secret.
    #[must_use]
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
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

    /// Returns PKCE code verifier.
    #[must_use]
    pub fn code_verifier(&self) -> Option<&str> {
        self.code_verifier.as_deref()
    }

    /// Returns OAuth client secret.
    #[must_use]
    pub fn client_secret(&self) -> Option<&str> {
        self.client_secret.as_deref()
    }
}
