use localid_authorization::AuthorizationContext;
use localid_identity::IdentityId;
use localid_session::SessionId;

use crate::authorization::{error::AuthorizationApplicationError, port::IdentityRolePort};

/// Resolves an authorization context from identity information.
pub struct AuthorizationContextResolver<R> {
    role_repository: R,
}

impl<R> AuthorizationContextResolver<R> {
    /// Creates a new authorization context resolver.
    #[must_use]
    pub const fn new(role_repository: R) -> Self {
        Self { role_repository }
    }
}

impl<R> AuthorizationContextResolver<R>
where
    R: IdentityRolePort,
{
    /// Resolves authorization context.
    pub fn resolve(
        &self,
        identity_id: IdentityId,
        session_id: SessionId,
    ) -> Result<AuthorizationContext, AuthorizationApplicationError> {
        let roles = self
            .role_repository
            .find_roles(identity_id)
            .map_err(|_| AuthorizationApplicationError::RoleResolutionFailure)?;

        Ok(AuthorizationContext::new(identity_id, session_id, roles))
    }
}
