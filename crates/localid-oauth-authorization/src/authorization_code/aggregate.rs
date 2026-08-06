use chrono::{DateTime, Utc};

use localid_identity::IdentityId;
use localid_oauth_client::OAuthClientId;

use super::{AuthorizationCodeError, AuthorizationCodeId, AuthorizationCodeLifecycleState};

/// Authorization code aggregate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationCode {
    id: AuthorizationCodeId,
    client_id: OAuthClientId,
    identity_id: IdentityId,
    code_hash: String,
    redirect_uri: String,
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
            created_at,
            expires_at,
            state: AuthorizationCodeLifecycleState::Active,
        })
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
}
