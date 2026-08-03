use std::collections::HashMap;

use localid_identity::{Identity, IdentityId};
use localid_repository::IdentityRepository;

use crate::MemoryRepositoryError;

/// In-memory identity repository.
#[derive(Debug, Default)]
pub struct MemoryIdentityRepository {
    identities: HashMap<IdentityId, Identity>,
}

impl MemoryIdentityRepository {
    /// Creates empty repository.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl IdentityRepository for MemoryIdentityRepository {
    type Error = MemoryRepositoryError;

    fn find_by_id(&self, id: IdentityId) -> Result<Option<Identity>, Self::Error> {
        Ok(self.identities.get(&id).cloned())
    }

    fn save(&mut self, identity: Identity) -> Result<(), Self::Error> {
        self.identities.insert(identity.id(), identity);

        Ok(())
    }
}
