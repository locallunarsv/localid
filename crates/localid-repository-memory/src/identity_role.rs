use std::collections::HashMap;

use localid_identity::IdentityId;
use localid_repository::IdentityRoleRepository;
use localid_role::Role;

use crate::error::MemoryRepositoryError;

/// In-memory identity role repository.
#[derive(Debug, Default)]
pub struct MemoryIdentityRoleRepository {
    roles: HashMap<IdentityId, Vec<Role>>,
}

impl MemoryIdentityRoleRepository {
    /// Creates an empty repository.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl IdentityRoleRepository for MemoryIdentityRoleRepository {
    type Error = MemoryRepositoryError;

    fn find_roles(&self, identity_id: IdentityId) -> Result<Vec<Role>, Self::Error> {
        Ok(self.roles.get(&identity_id).cloned().unwrap_or_default())
    }

    fn assign(&mut self, identity_id: IdentityId, roles: Vec<Role>) -> Result<(), Self::Error> {
        self.roles.insert(identity_id, roles);

        Ok(())
    }
}
