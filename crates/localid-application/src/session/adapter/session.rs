use localid_authentication::SessionService;
use localid_session::{Session, SessionId};

use crate::session::port::SessionPort;

/// Adapter for session management service.
#[derive(Debug, Clone, Copy)]
pub struct SessionAdapter<S> {
    service: S,
}

impl<S> SessionAdapter<S> {
    /// Creates a new session adapter.
    #[must_use]
    pub const fn new(service: S) -> Self {
        Self { service }
    }
}

impl<S> SessionPort for SessionAdapter<S>
where
    S: SessionService,
{
    type Error = S::Error;

    fn find(&mut self, session_id: SessionId) -> Result<Session, Self::Error> {
        self.service.find(session_id)
    }

    fn revoke(&mut self, session_id: SessionId) -> Result<(), Self::Error> {
        self.service.revoke(session_id)
    }
}
