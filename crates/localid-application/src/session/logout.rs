use localid_session::SessionId;

use super::SessionPort;

use crate::ApplicationError;

/// Use case for revoking an authenticated session.
pub struct LogoutSessionUseCase<P> {
    session_port: P,
}

impl<P> LogoutSessionUseCase<P> {
    /// Creates a logout session use case.
    #[must_use]
    pub const fn new(session_port: P) -> Self {
        Self { session_port }
    }
}

impl<P> LogoutSessionUseCase<P>
where
    P: SessionPort,
{
    /// Revokes a session.
    ///
    /// # Errors
    ///
    /// Returns an application error when session revocation fails.
    pub fn execute(&mut self, session_id: SessionId) -> Result<(), ApplicationError> {
        self.session_port
            .revoke(session_id)
            .map_err(|_| ApplicationError::InternalFailure)
    }
}
