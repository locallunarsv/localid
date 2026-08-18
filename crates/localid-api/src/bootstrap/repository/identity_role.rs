use localid_identity::IdentityId;
use localid_repository::IdentityRoleRepository;
use localid_role::Role;

use super::SharedRepository;

impl<T> IdentityRoleRepository for SharedRepository<T>
where
    T: IdentityRoleRepository,
{
    type Error = T::Error;

    fn find_roles(&self, identity_id: IdentityId) -> Result<Vec<Role>, Self::Error> {
        self.with(|repository| repository.find_roles(identity_id))
    }

    fn assign(&mut self, identity_id: IdentityId, roles: Vec<Role>) -> Result<(), Self::Error> {
        self.with_mut(|repository| repository.assign(identity_id, roles))
    }
}
