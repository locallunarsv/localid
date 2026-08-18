use chrono::{DateTime, Utc};
use localid_session::SessionId;

use super::{TokenError, TokenId, TokenLifecycleState};

/// Client-facing credential that grants access to an authenticated Session.
///
/// Token owns its lifecycle and validity period. Transport representation,
/// authorization rules, and token generation policies remain outside this
/// domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    id: TokenId,
    session_id: SessionId,
    secret_hash: String,
    lifecycle_state: TokenLifecycleState,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

impl Token {
    /// Creates a new active Token.
    ///
    /// # Errors
    ///
    /// Returns [`TokenError::InvalidExpirationTime`] when `expires_at` is not
    /// after `created_at`.
    pub fn new(
        id: TokenId,
        session_id: SessionId,
        secret_hash: String,
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Result<Self, TokenError> {
        if expires_at <= created_at {
            return Err(TokenError::InvalidExpirationTime);
        }

        Ok(Self {
            id,
            session_id,
            secret_hash,
            lifecycle_state: TokenLifecycleState::Active,
            created_at,
            expires_at,
        })
    }

    /// Restores a Token aggregate from persistent storage.
    #[must_use]
    pub const fn restore(
        id: TokenId,
        session_id: SessionId,
        secret_hash: String,
        lifecycle_state: TokenLifecycleState,
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            session_id,
            secret_hash,
            lifecycle_state,
            created_at,
            expires_at,
        }
    }

    /// Returns the Token identifier.
    #[must_use]
    pub const fn id(&self) -> TokenId {
        self.id
    }

    /// Returns the associated Session identifier.
    #[must_use]
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    /// Returns the stored secret hash.
    #[must_use]
    pub fn secret_hash(&self) -> &str {
        &self.secret_hash
    }

    /// Returns the current lifecycle state.
    #[must_use]
    pub const fn lifecycle_state(&self) -> TokenLifecycleState {
        self.lifecycle_state
    }

    /// Returns creation time.
    #[must_use]
    pub const fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Returns expiration time.
    #[must_use]
    pub const fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    /// Returns true when Token has not been revoked.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.lifecycle_state.is_active()
    }

    /// Returns true when Token has been revoked.
    #[must_use]
    pub const fn is_revoked(&self) -> bool {
        self.lifecycle_state.is_revoked()
    }

    /// Returns true when Token has expired at a specific time.
    #[must_use]
    pub fn is_expired_at(&self, now: DateTime<Utc>) -> bool {
        now >= self.expires_at
    }

    /// Returns true when Token can currently be used.
    #[must_use]
    pub fn is_valid_at(&self, now: DateTime<Utc>) -> bool {
        self.is_active() && !self.is_expired_at(now)
    }

    /// Revokes this Token permanently.
    pub const fn revoke(&mut self) {
        self.lifecycle_state = TokenLifecycleState::Revoked;
    }
}
