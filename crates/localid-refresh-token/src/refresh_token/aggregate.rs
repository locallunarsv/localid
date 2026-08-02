use chrono::{DateTime, Utc};
use localid_session::SessionId;

use super::{RefreshTokenError, RefreshTokenId, RefreshTokenLifecycleState};

/// Long-lived credential used to obtain new access tokens.
///
/// RefreshToken stores only the hashed secret representation.
/// The raw secret is never persisted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefreshToken {
    id: RefreshTokenId,
    session_id: SessionId,
    secret_hash: String,
    lifecycle_state: RefreshTokenLifecycleState,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

impl RefreshToken {
    /// Creates a new active Refresh Token.
    ///
    /// # Errors
    ///
    /// Returns [`RefreshTokenError::InvalidExpirationTime`] when
    /// expiration does not occur after creation.
    pub fn new(
        id: RefreshTokenId,
        session_id: SessionId,
        secret_hash: String,
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Result<Self, RefreshTokenError> {
        if expires_at <= created_at {
            return Err(RefreshTokenError::InvalidExpirationTime);
        }

        Ok(Self {
            id,
            session_id,
            secret_hash,
            lifecycle_state: RefreshTokenLifecycleState::Active,
            created_at,
            expires_at,
        })
    }

    /// Returns the Refresh Token identifier.
    #[must_use]
    pub const fn id(&self) -> RefreshTokenId {
        self.id
    }

    /// Returns the associated session identifier.
    #[must_use]
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    /// Returns the stored secret hash.
    #[must_use]
    pub fn secret_hash(&self) -> &str {
        &self.secret_hash
    }

    /// Returns the lifecycle state.
    #[must_use]
    pub const fn lifecycle_state(&self) -> RefreshTokenLifecycleState {
        self.lifecycle_state
    }

    /// Returns whether this token is active.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.lifecycle_state.is_active()
    }

    /// Returns whether this token has expired.
    #[must_use]
    pub fn is_expired_at(&self, now: DateTime<Utc>) -> bool {
        now >= self.expires_at
    }

    /// Returns whether this token can be used.
    #[must_use]
    pub fn is_valid_at(&self, now: DateTime<Utc>) -> bool {
        self.is_active() && !self.is_expired_at(now)
    }

    /// Revokes this Refresh Token.
    pub const fn revoke(&mut self) {
        self.lifecycle_state = RefreshTokenLifecycleState::Revoked;
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeDelta, TimeZone, Utc};
    use localid_session::SessionId;

    use super::RefreshToken;
    use crate::{RefreshTokenError, RefreshTokenId, RefreshTokenLifecycleState};

    fn creation_time() -> chrono::DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 8, 2, 0, 0, 0)
            .single()
            .expect("test timestamp should be valid")
    }

    fn refresh_token() -> RefreshToken {
        let created_at = creation_time();

        RefreshToken::new(
            RefreshTokenId::new(),
            SessionId::new(),
            "hashed-refresh-secret".to_owned(),
            created_at,
            created_at + TimeDelta::days(30),
        )
        .expect("refresh token should be valid")
    }

    #[test]
    fn creates_active_refresh_token() {
        let token = refresh_token();

        assert!(token.is_active());
        assert!(!token.is_expired_at(creation_time()));
        assert_eq!(token.lifecycle_state(), RefreshTokenLifecycleState::Active);
    }

    #[test]
    fn rejects_invalid_expiration_time() {
        let created_at = creation_time();

        let result = RefreshToken::new(
            RefreshTokenId::new(),
            SessionId::new(),
            "hash".to_owned(),
            created_at,
            created_at,
        );

        assert_eq!(result, Err(RefreshTokenError::InvalidExpirationTime));
    }

    #[test]
    fn refresh_token_expires_at_expiration_time() {
        let token = refresh_token();

        assert!(token.is_expired_at(creation_time() + TimeDelta::days(30)));
    }

    #[test]
    fn revokes_refresh_token() {
        let mut token = refresh_token();

        token.revoke();

        assert!(!token.is_active());
        assert!(token.lifecycle_state().is_revoked());
    }
}
