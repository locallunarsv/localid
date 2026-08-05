use localid_session::SessionId;

use crate::{
    error::ApplicationError,
    session::{SessionPort, SessionResponse},
};

/// Use case for retrieving current session information.
pub struct GetCurrentSessionUseCase<P> {
    session_port: P,
}

impl<P> GetCurrentSessionUseCase<P> {
    /// Creates a new current session use case.
    #[must_use]
    pub const fn new(session_port: P) -> Self {
        Self { session_port }
    }
}

impl<P> GetCurrentSessionUseCase<P>
where
    P: SessionPort<Error = localid_authentication::AuthenticationError>,
{
    /// Retrieves a session by identifier.
    pub fn execute(&mut self, session_id: SessionId) -> Result<SessionResponse, ApplicationError> {
        let session = self
            .session_port
            .find(session_id)
            .map_err(ApplicationError::from)?;

        Ok(SessionResponse::from(session))
    }
}
