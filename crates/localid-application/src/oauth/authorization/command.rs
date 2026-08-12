use localid_identity::IdentityId;
use localid_oauth_authorization::CodeChallengeMethod;

/// OAuth authorization request command.
#[derive(Debug, Clone)]
pub struct AuthorizeCommand {
    client_id: String,
    identity_id: IdentityId,
    redirect_uri: String,
    scope: Vec<String>,
    nonce: Option<String>,
    request_state: Option<String>,
    code_challenge: Option<String>,
    code_challenge_method: Option<CodeChallengeMethod>,
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
        Self::new_with_nonce(client_id, identity_id, redirect_uri, scope, None, None)
    }

    /// Creates a new authorization command with OIDC nonce.
    pub fn new_with_nonce(
        client_id: impl Into<String>,
        identity_id: IdentityId,
        redirect_uri: impl Into<String>,
        scope: Vec<String>,
        nonce: Option<String>,
        request_state: Option<String>,
    ) -> Self {
        Self::new_with_nonce_and_pkce(
            client_id,
            identity_id,
            redirect_uri,
            scope,
            nonce,
            request_state,
            None,
            None,
        )
    }

    /// Creates a new authorization command with OIDC nonce and PKCE parameters.
    #[must_use]
    pub fn new_with_nonce_and_pkce(
        client_id: impl Into<String>,
        identity_id: IdentityId,
        redirect_uri: impl Into<String>,
        scope: Vec<String>,
        nonce: Option<String>,
        request_state: Option<String>,
        code_challenge: Option<String>,
        code_challenge_method: Option<CodeChallengeMethod>,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            identity_id,
            redirect_uri: redirect_uri.into(),
            scope,
            nonce,
            request_state,
            code_challenge,
            code_challenge_method,
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

    /// Returns OpenID Connect nonce.
    #[must_use]
    pub fn nonce(&self) -> Option<&str> {
        self.nonce.as_deref()
    }

    /// Returns OAuth state parameter.
    #[must_use]
    pub fn request_state(&self) -> Option<&str> {
        self.request_state.as_deref()
    }

    /// Returns PKCE code challenge.
    #[must_use]
    pub fn code_challenge(&self) -> Option<&str> {
        self.code_challenge.as_deref()
    }

    /// Returns PKCE code challenge method.
    #[must_use]
    pub const fn code_challenge_method(&self) -> Option<&CodeChallengeMethod> {
        self.code_challenge_method.as_ref()
    }
}
