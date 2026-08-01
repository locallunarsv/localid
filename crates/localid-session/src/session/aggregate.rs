use chrono::{DateTime, Utc};
use localid_identity::IdentityId;

use super::{SessionError, SessionId, SessionLifecycleState};

/// Authenticated interaction associated with exactly one Identity.
///
/// A Session owns its lifecycle and validity period. Authentication,
/// authorization, tokens, and transport-specific representations remain
/// outside this domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    id: SessionId,
    identity_id: IdentityId,
    lifecycle_state: SessionLifecycleState,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

impl Session {
    /// Creates a new active Session.
    ///
    /// # Errors
    ///
    /// Returns [`SessionError::InvalidExpirationTime`] when `expires_at`
    /// does not occur strictly after `created_at`.
    pub fn new(
        id: SessionId,
        identity_id: IdentityId,
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Result<Self, SessionError> {
        if expires_at <= created_at {
            return Err(SessionError::InvalidExpirationTime);
        }

        Ok(Self {
            id,
            identity_id,
            lifecycle_state: SessionLifecycleState::INITIAL,
            created_at,
            expires_at,
        })
    }

    /// Returns this Session's stable identifier.
    #[must_use]
    pub const fn id(&self) -> SessionId {
        self.id
    }

    /// Returns the identifier of the Identity that owns this Session.
    #[must_use]
    pub const fn identity_id(&self) -> IdentityId {
        self.identity_id
    }

    /// Returns the current lifecycle state.
    #[must_use]
    pub const fn lifecycle_state(&self) -> SessionLifecycleState {
        self.lifecycle_state
    }

    /// Returns the time at which this Session was created.
    #[must_use]
    pub const fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Returns the time at which this Session expires.
    #[must_use]
    pub const fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    /// Returns `true` when this Session has not been revoked.
    ///
    /// This method checks lifecycle state only. It does not evaluate
    /// time-based expiration.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.lifecycle_state.is_active()
    }

    /// Returns `true` when this Session has been revoked.
    #[must_use]
    pub const fn is_revoked(&self) -> bool {
        self.lifecycle_state.is_revoked()
    }

    /// Returns `true` when `now` is at or after the expiration time.
    #[must_use]
    pub fn is_expired_at(&self, now: DateTime<Utc>) -> bool {
        now >= self.expires_at
    }

    /// Returns `true` when this Session is active and has not expired.
    #[must_use]
    pub fn is_valid_at(&self, now: DateTime<Utc>) -> bool {
        self.is_active() && !self.is_expired_at(now)
    }

    /// Permanently revokes this Session.
    ///
    /// Revoking an already revoked Session is idempotent.
    pub const fn revoke(&mut self) {
        self.lifecycle_state = SessionLifecycleState::Revoked;
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeDelta, TimeZone, Utc};
    use localid_identity::IdentityId;

    use super::Session;
    use crate::{SessionError, SessionId, SessionLifecycleState};

    fn creation_time() -> chrono::DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 8, 2, 0, 0, 0)
            .single()
            .expect("test timestamp should be valid")
    }

    #[test]
    fn creates_active_session() {
        let id = SessionId::new();
        let identity_id = IdentityId::new();
        let created_at = creation_time();
        let expires_at = created_at + TimeDelta::hours(1);

        let session = Session::new(id, identity_id, created_at, expires_at)
            .expect("expiration after creation should be valid");

        assert_eq!(session.id(), id);
        assert_eq!(session.identity_id(), identity_id);
        assert_eq!(session.lifecycle_state(), SessionLifecycleState::Active);
        assert_eq!(session.created_at(), created_at);
        assert_eq!(session.expires_at(), expires_at);
        assert!(session.is_active());
        assert!(!session.is_revoked());
    }

    #[test]
    fn rejects_expiration_equal_to_creation() {
        let created_at = creation_time();

        let result = Session::new(SessionId::new(), IdentityId::new(), created_at, created_at);

        assert_eq!(result, Err(SessionError::InvalidExpirationTime));
    }

    #[test]
    fn rejects_expiration_before_creation() {
        let created_at = creation_time();
        let expires_at = created_at - TimeDelta::seconds(1);

        let result = Session::new(SessionId::new(), IdentityId::new(), created_at, expires_at);

        assert_eq!(result, Err(SessionError::InvalidExpirationTime));
    }

    #[test]
    fn session_is_valid_before_expiration() {
        let created_at = creation_time();
        let expires_at = created_at + TimeDelta::hours(1);

        let session = Session::new(SessionId::new(), IdentityId::new(), created_at, expires_at)
            .expect("expiration after creation should be valid");

        assert!(session.is_valid_at(created_at));
        assert!(session.is_valid_at(expires_at - TimeDelta::seconds(1)));
    }

    #[test]
    fn session_expires_at_expiration_time() {
        let created_at = creation_time();
        let expires_at = created_at + TimeDelta::hours(1);

        let session = Session::new(SessionId::new(), IdentityId::new(), created_at, expires_at)
            .expect("expiration after creation should be valid");

        assert!(session.is_expired_at(expires_at));
        assert!(!session.is_valid_at(expires_at));
    }

    #[test]
    fn revokes_active_session() {
        let created_at = creation_time();
        let expires_at = created_at + TimeDelta::hours(1);

        let mut session = Session::new(SessionId::new(), IdentityId::new(), created_at, expires_at)
            .expect("expiration after creation should be valid");

        session.revoke();

        assert!(session.is_revoked());
        assert!(!session.is_valid_at(created_at));
    }

    #[test]
    fn revoking_revoked_session_is_idempotent() {
        let created_at = creation_time();
        let expires_at = created_at + TimeDelta::hours(1);

        let mut session = Session::new(SessionId::new(), IdentityId::new(), created_at, expires_at)
            .expect("expiration after creation should be valid");

        session.revoke();
        session.revoke();

        assert!(session.is_revoked());
    }
}
