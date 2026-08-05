use localid_identity::IdentityId;
use localid_role::Role;

/// Role lookup capability required by authorization use cases.
pub trait IdentityRolePort {
    /// Error returned by role lookup.
    type Error;

    /// Finds roles assigned to an identity.
    fn find_roles(&self, identity_id: IdentityId) -> Result<Vec<Role>, Self::Error>;
}
