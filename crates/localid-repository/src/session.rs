use localid_identity::IdentityId;
use localid_session::{Session, SessionId};

/// Repository for Session aggregates.
pub trait SessionRepository {
    /// Repository-specific error.
    type Error;

    /// Finds a Session by its identifier.
    fn find_by_id(&self, id: SessionId) -> Result<Option<Session>, Self::Error>;

    /// Finds all Sessions owned by an Identity.
    fn find_by_identity_id(&self, identity_id: IdentityId) -> Result<Vec<Session>, Self::Error>;

    /// Persists a Session.
    fn save(&mut self, session: Session) -> Result<(), Self::Error>;
}
