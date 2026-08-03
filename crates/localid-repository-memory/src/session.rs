use std::collections::HashMap;

use localid_identity::IdentityId;
use localid_repository::SessionRepository;
use localid_session::{Session, SessionId};

use crate::MemoryRepositoryError;

/// In-memory session repository.
#[derive(Debug, Default)]
pub struct MemorySessionRepository {
    sessions: HashMap<SessionId, Session>,
}

impl MemorySessionRepository {
    /// Creates an empty session repository.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl SessionRepository for MemorySessionRepository {
    type Error = MemoryRepositoryError;

    fn find_by_id(&self, id: SessionId) -> Result<Option<Session>, Self::Error> {
        Ok(self.sessions.get(&id).cloned())
    }

    fn find_by_identity_id(&self, identity_id: IdentityId) -> Result<Vec<Session>, Self::Error> {
        Ok(self
            .sessions
            .values()
            .filter(|session| session.identity_id() == identity_id)
            .cloned()
            .collect())
    }

    fn save(&mut self, session: Session) -> Result<(), Self::Error> {
        self.sessions.insert(session.id(), session);

        Ok(())
    }
}
