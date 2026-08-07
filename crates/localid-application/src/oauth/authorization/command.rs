use localid_identity::IdentityId;

/// OAuth authorization request command.
#[derive(Debug, Clone)]
pub struct AuthorizeCommand {
    client_id: String,
    identity_id: IdentityId,
    redirect_uri: String,
    scope: Vec<String>,
}

impl AuthorizeCommand {
    /// Creates a new authorization command.
    #[must_use]
    pub fn new(
        client_id: impl Into<String>,
        identity_id: IdentityId,
        redirect_uri: impl Into<String>,
        scope: Vec<String>,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            identity_id,
            redirect_uri: redirect_uri.into(),
            scope,
        }
    }

    /// Returns OAuth client public identifier.
    #[must_use]
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Returns authenticated identity identifier.
    #[must_use]
    pub const fn identity_id(&self) -> IdentityId {
        self.identity_id
    }

    /// Returns registered redirect URI.
    #[must_use]
    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }

    /// Returns requested OAuth scopes.
    #[must_use]
    pub fn scope(&self) -> &[String] {
        &self.scope
    }
}
