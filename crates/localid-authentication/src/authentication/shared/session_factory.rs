use chrono::{Duration, Utc};

use localid_identity::IdentityId;
use localid_session::{Session, SessionId};

/// Creates authentication sessions.
pub trait SessionFactory {
    /// Error returned when session creation fails.
    type Error;

    /// Creates a new session.
    fn create_session(&self, identity_id: IdentityId) -> Result<Session, Self::Error>;
}

/// Default session factory.
#[derive(Debug, Clone, Copy)]
pub struct DefaultSessionFactory;

impl DefaultSessionFactory {
    /// Creates a default session factory.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for DefaultSessionFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionFactory for DefaultSessionFactory {
    type Error = localid_session::SessionError;

    fn create_session(&self, identity_id: IdentityId) -> Result<Session, Self::Error> {
        let created_at = Utc::now();

        Session::new(
            SessionId::new(),
            identity_id,
            created_at,
            created_at + Duration::hours(1),
        )
    }
}
