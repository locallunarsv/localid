use localid_identity::IdentityId;
use localid_session::SessionId;

/// Context produced after successful token validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthenticatedContext {
    identity_id: IdentityId,
    session_id: SessionId,
}

impl AuthenticatedContext {
    /// Creates an authenticated context.
    #[must_use]
    pub const fn new(identity_id: IdentityId, session_id: SessionId) -> Self {
        Self {
            identity_id,
            session_id,
        }
    }

    /// Returns the authenticated Identity identifier.
    #[must_use]
    pub const fn identity_id(&self) -> IdentityId {
        self.identity_id
    }

    /// Returns the authenticated Session identifier.
    #[must_use]
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }
}

/// Contract for validating authentication tokens.
pub trait TokenValidator {
    /// Error returned during validation.
    type Error;

    /// Validates a raw token secret.
    fn validate(&self, secret: &str) -> Result<AuthenticatedContext, Self::Error>;
}
