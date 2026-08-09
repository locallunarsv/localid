use localid_oauth_authorization::AuthorizationCodeId;

/// OAuth token exchange request command.
#[derive(Debug, Clone)]
pub struct TokenExchangeCommand {
    code_id: AuthorizationCodeId,
    client_id: String,
    redirect_uri: String,
}

impl TokenExchangeCommand {
    /// Creates a new token exchange command.
    #[must_use]
    pub fn new(
        code_id: AuthorizationCodeId,
        client_id: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        Self {
            code_id,
            client_id: client_id.into(),
            redirect_uri: redirect_uri.into(),
        }
    }

    /// Returns authorization code identifier.
    #[must_use]
    pub const fn code_id(&self) -> AuthorizationCodeId {
        self.code_id
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
