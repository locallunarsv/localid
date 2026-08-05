use localid_identity::IdentityId;
use localid_repository::IdentityRoleRepository;
use localid_role::Role;

use crate::authorization::IdentityRolePort;

/// Adapter for identity role repository.
pub struct IdentityRoleAdapter<R> {
    repository: R,
}

impl<R> IdentityRoleAdapter<R> {
    /// Creates a new identity role adapter.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> IdentityRolePort for IdentityRoleAdapter<R>
where
    R: IdentityRoleRepository,
{
    type Error = R::Error;

    fn find_roles(&self, identity_id: IdentityId) -> Result<Vec<Role>, Self::Error> {
        self.repository.find_roles(identity_id)
    }
}
