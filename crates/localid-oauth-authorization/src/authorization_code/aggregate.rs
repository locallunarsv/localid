use chrono::{DateTime, Utc};

use localid_identity::IdentityId;
use localid_oauth_client::OAuthClientId;

use super::{
    AuthorizationCodeError, AuthorizationCodeId, AuthorizationCodeLifecycleState,
    CodeChallengeMethod,
};

/// Authorization code aggregate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationCode {
    id: AuthorizationCodeId,
    client_id: OAuthClientId,
    identity_id: IdentityId,
    code_hash: String,
    redirect_uri: String,

    nonce: Option<String>,
    scope: Vec<String>,
    request_state: Option<String>,

    pkce_challenge: Option<String>,
    pkce_method: Option<CodeChallengeMethod>,

    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    state: AuthorizationCodeLifecycleState,
}

impl AuthorizationCode {
    /// Creates a new authorization code.
    pub fn new(
        id: AuthorizationCodeId,
        client_id: OAuthClientId,
        identity_id: IdentityId,
        code_hash: impl Into<String>,
        redirect_uri: impl Into<String>,
        scope: Vec<String>,
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Result<Self, AuthorizationCodeError> {
        Self::new_with_nonce(
            id,
            client_id,
            identity_id,
            code_hash,
            redirect_uri,
            None,
            scope,
            None,
            created_at,
            expires_at,
        )
    }

    /// Creates a new authorization code with OIDC nonce.
    pub fn new_with_nonce(
        id: AuthorizationCodeId,
        client_id: OAuthClientId,
        identity_id: IdentityId,
        code_hash: impl Into<String>,
        redirect_uri: impl Into<String>,
        nonce: Option<String>,
        scope: Vec<String>,
        request_state: Option<String>,
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Result<Self, AuthorizationCodeError> {
        if expires_at <= created_at {
            return Err(AuthorizationCodeError::InvalidExpirationTime);
        }

        Self::new_with_nonce_and_pkce(
            id,
            client_id,
            identity_id,
            code_hash,
            redirect_uri,
            nonce,
            scope,
            request_state,
            None,
            None,
            created_at,
            expires_at,
        )
    }

    /// Creates authorization code with OIDC and PKCE data.
    pub fn new_with_nonce_and_pkce(
        id: AuthorizationCodeId,
        client_id: OAuthClientId,
        identity_id: IdentityId,
        code_hash: impl Into<String>,
        redirect_uri: impl Into<String>,
        nonce: Option<String>,
        scope: Vec<String>,
        request_state: Option<String>,
        pkce_challenge: Option<String>,
        pkce_method: Option<CodeChallengeMethod>,
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Result<Self, AuthorizationCodeError> {
        if expires_at <= created_at {
            return Err(AuthorizationCodeError::InvalidExpirationTime);
        }

        Ok(Self {
            id,
            client_id,
            identity_id,
            code_hash: code_hash.into(),
            redirect_uri: redirect_uri.into(),
            nonce,
            scope,
            request_state,
            pkce_challenge,
            pkce_method,
            created_at,
            expires_at,
            state: AuthorizationCodeLifecycleState::Active,
        })
    }

    /// Restores an authorization code aggregate from persistent storage.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn restore(
        id: AuthorizationCodeId,
        client_id: OAuthClientId,
        identity_id: IdentityId,
        code_hash: String,
        redirect_uri: String,
        nonce: Option<String>,
        scope: Vec<String>,
        request_state: Option<String>,
        pkce_challenge: Option<String>,
        pkce_method: Option<CodeChallengeMethod>,
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
        state: AuthorizationCodeLifecycleState,
    ) -> Self {
        Self {
            id,
            client_id,
            identity_id,
            code_hash,
            redirect_uri,
            nonce,
            scope,
            request_state,
            pkce_challenge,
            pkce_method,
            created_at,
            expires_at,
            state,
        }
    }

    #[must_use]
    pub const fn id(&self) -> AuthorizationCodeId {
        self.id
    }

    #[must_use]
    pub const fn client_id(&self) -> OAuthClientId {
        self.client_id
    }

    #[must_use]
    pub const fn identity_id(&self) -> IdentityId {
        self.identity_id
    }

    #[must_use]
    pub fn code_hash(&self) -> &str {
        &self.code_hash
    }

    #[must_use]
    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }

    /// Returns the OIDC nonce.
    #[must_use]
    pub fn nonce(&self) -> Option<&str> {
        self.nonce.as_deref()
    }

    /// Returns PKCE code challenge.
    #[must_use]
    pub fn pkce_challenge(&self) -> Option<&str> {
        self.pkce_challenge.as_deref()
    }

    /// Returns PKCE challenge method.
    #[must_use]
    pub const fn pkce_method(&self) -> Option<&CodeChallengeMethod> {
        self.pkce_method.as_ref()
    }

    /// Returns granted OAuth scopes.
    #[must_use]
    pub fn scope(&self) -> &[String] {
        &self.scope
    }

    #[must_use]
    pub const fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    #[must_use]
    pub const fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    #[must_use]
    pub const fn state(&self) -> AuthorizationCodeLifecycleState {
        self.state
    }

    /// Consumes authorization code.
    pub fn consume(&mut self) -> Result<(), AuthorizationCodeError> {
        if self.state == AuthorizationCodeLifecycleState::Consumed {
            return Err(AuthorizationCodeError::AlreadyConsumed);
        }

        self.state = AuthorizationCodeLifecycleState::Consumed;

        Ok(())
    }

    #[must_use]
    pub fn is_active_at(&self, time: DateTime<Utc>) -> bool {
        self.state.is_active() && time < self.expires_at
    }

    #[must_use]
    pub fn is_expired_at(&self, time: DateTime<Utc>) -> bool {
        time >= self.expires_at
    }

    /// Returns OAuth request state.
    #[must_use]
    pub fn request_state(&self) -> Option<&str> {
        self.request_state.as_deref()
    }
}
