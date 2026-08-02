use localid_identity::IdentityId;
use localid_session::SessionId;

use super::Role;

/// Context used during authorization evaluation.
///
/// AuthorizationContext represents the authenticated subject and
/// its assigned roles at authorization time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationContext {
    identity_id: IdentityId,
    session_id: SessionId,
    roles: Vec<Role>,
}

impl AuthorizationContext {
    /// Creates a new authorization context.
    #[must_use]
    pub fn new(identity_id: IdentityId, session_id: SessionId, roles: Vec<Role>) -> Self {
        Self {
            identity_id,
            session_id,
            roles,
        }
    }

    /// Returns the Identity identifier.
    #[must_use]
    pub const fn identity_id(&self) -> IdentityId {
        self.identity_id
    }

    /// Returns the Session identifier.
    #[must_use]
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    /// Returns roles assigned to this context.
    #[must_use]
    pub fn roles(&self) -> &[Role] {
        &self.roles
    }
}

#[cfg(test)]
mod tests {
    use localid_identity::IdentityId;
    use localid_session::SessionId;

    use super::AuthorizationContext;

    #[test]
    fn creates_authorization_context() {
        let context = AuthorizationContext::new(IdentityId::new(), SessionId::new(), Vec::new());

        assert!(context.roles().is_empty());
    }
}
