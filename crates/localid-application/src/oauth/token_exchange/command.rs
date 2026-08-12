/// OAuth token exchange request command.
#[derive(Debug, Clone)]
pub struct TokenExchangeCommand {
    code: String,
    client_id: String,
    redirect_uri: String,
    code_verifier: Option<String>,
}

impl TokenExchangeCommand {
    /// Creates a new token exchange command.
    #[must_use]
    pub fn new(
        code: impl Into<String>,
        client_id: impl Into<String>,
        redirect_uri: impl Into<String>,
        code_verifier: Option<String>,
    ) -> Self {
        Self {
            code: code.into(),
            client_id: client_id.into(),
            redirect_uri: redirect_uri.into(),
            code_verifier,
        }
    }

    /// Returns authorization code secret.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
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

    /// Returns PKCE code verifier.
    #[must_use]
    pub fn code_verifier(&self) -> Option<&str> {
        self.code_verifier.as_deref()
    }
}
