use localid_session::{Session, SessionId};

/// Session management capability required by session use cases.
pub trait SessionPort {
    /// Error returned during session operations.
    type Error;

    /// Finds a session by identifier.
    fn find(&mut self, session_id: SessionId) -> Result<Session, Self::Error>;

    /// Revokes a session by identifier.
    fn revoke(&mut self, session_id: SessionId) -> Result<(), Self::Error>;
}
