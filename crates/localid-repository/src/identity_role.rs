use localid_identity::IdentityId;
use localid_role::Role;

/// Repository for resolving roles assigned to an identity.
pub trait IdentityRoleRepository {
    /// Repository error type.
    type Error;

    /// Finds roles assigned to an identity.
    fn find_roles(&self, identity_id: IdentityId) -> Result<Vec<Role>, Self::Error>;
}
