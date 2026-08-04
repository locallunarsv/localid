use localid_identity::IdentityId;
use localid_session::SessionId;

/// Represents an authenticated identity context.
#[derive(Debug, Clone, Copy)]
pub struct IdentityContext {
    identity_id: IdentityId,
    session_id: SessionId,
}

impl IdentityContext {
    /// Creates a new identity context.
    #[must_use]
    pub const fn new(identity_id: IdentityId, session_id: SessionId) -> Self {
        Self {
            identity_id,
            session_id,
        }
    }

    /// Returns identity identifier.
    #[must_use]
    pub const fn identity_id(&self) -> IdentityId {
        self.identity_id
    }

    /// Returns session identifier.
    #[must_use]
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }
}
