use localid_identity::IdentityId;
use localid_session::SessionId;

/// Result of successful access token verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenVerificationResult {
    identity_id: IdentityId,
    session_id: SessionId,
}

impl TokenVerificationResult {
    /// Creates a token verification result.
    #[must_use]
    pub const fn new(identity_id: IdentityId, session_id: SessionId) -> Self {
        Self {
            identity_id,
            session_id,
        }
    }

    /// Returns authenticated identity id.
    #[must_use]
    pub const fn identity_id(&self) -> IdentityId {
        self.identity_id
    }

    /// Returns session id.
    #[must_use]
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }
}
