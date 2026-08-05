use localid_session::{Session, SessionId};

/// Session management service.
pub trait SessionService {
    /// Error returned during session operations.
    type Error;

    /// Finds a session by identifier.
    fn find(&mut self, session_id: SessionId) -> Result<Session, Self::Error>;

    /// Revokes a session.
    fn revoke(&mut self, session_id: SessionId) -> Result<(), Self::Error>;
}
