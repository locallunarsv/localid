use localid_identity::IdentityId;
use localid_session::SessionId;

/// Response returned after successful token verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerifyTokenResponse {
    identity_id: IdentityId,
    session_id: SessionId,
}

impl VerifyTokenResponse {
    /// Creates a token verification response.
    #[must_use]
    pub const fn new(identity_id: IdentityId, session_id: SessionId) -> Self {
        Self {
            identity_id,
            session_id,
        }
    }

    /// Returns identity id.
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
