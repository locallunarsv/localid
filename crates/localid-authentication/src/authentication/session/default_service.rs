use localid_repository::SessionRepository;
use localid_session::{Session, SessionId};

use super::service::SessionService;

use crate::AuthenticationError;

/// Default session management service.
pub struct DefaultSessionService<SR> {
    session_repository: SR,
}

impl<SR> DefaultSessionService<SR> {
    /// Creates a new session service.
    #[must_use]
    pub const fn new(session_repository: SR) -> Self {
        Self { session_repository }
    }
}

impl<SR> SessionService for DefaultSessionService<SR>
where
    SR: SessionRepository,
{
    type Error = AuthenticationError;

    fn find(&mut self, session_id: SessionId) -> Result<Session, Self::Error> {
        self.session_repository
            .find_by_id(session_id)
            .map_err(|_| AuthenticationError::SessionRepositoryFailure)?
            .ok_or(AuthenticationError::SessionNotFound)
    }

    fn revoke(&mut self, session_id: SessionId) -> Result<(), Self::Error> {
        let mut session = self.find(session_id)?;

        session.revoke();

        self.session_repository
            .save(session)
            .map_err(|_| AuthenticationError::SessionRepositoryFailure)
    }
}
