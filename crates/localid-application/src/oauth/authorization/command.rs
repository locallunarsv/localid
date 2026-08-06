use localid_client::ClientId;
use localid_identity::IdentityId;

/// OAuth authorization request command.
#[derive(Debug, Clone)]
pub struct AuthorizeCommand {
    client_id: ClientId,
    identity_id: IdentityId,
    redirect_uri: String,
    scope: Vec<String>,
}

impl AuthorizeCommand {
    /// Creates a new authorization command.
    #[must_use]
    pub fn new(
        client_id: ClientId,
        identity_id: IdentityId,
        redirect_uri: impl Into<String>,
        scope: Vec<String>,
    ) -> Self {
        Self {
            client_id,
            identity_id,
            redirect_uri: redirect_uri.into(),
            scope,
        }
    }

    /// Returns OAuth client identifier.
    #[must_use]
    pub const fn client_id(&self) -> ClientId {
        self.client_id
    }

    /// Returns authenticated identity identifier.
    #[must_use]
    pub const fn identity_id(&self) -> IdentityId {
        self.identity_id
    }

    /// Returns redirect URI.
    #[must_use]
    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }

    /// Returns requested scopes.
    #[must_use]
    pub fn scope(&self) -> &[String] {
        &self.scope
    }
}
