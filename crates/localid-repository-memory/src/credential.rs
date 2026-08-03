use std::collections::HashMap;

use localid_credential::{Credential, CredentialId};
use localid_identity::IdentityId;
use localid_repository::CredentialRepository;

use crate::MemoryRepositoryError;

/// In-memory credential repository.
#[derive(Debug, Default)]
pub struct MemoryCredentialRepository {
    credentials: HashMap<CredentialId, Credential>,
}

impl MemoryCredentialRepository {
    /// Creates empty repository.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl CredentialRepository for MemoryCredentialRepository {
    type Error = MemoryRepositoryError;

    fn find_by_id(&self, id: CredentialId) -> Result<Option<Credential>, Self::Error> {
        Ok(self.credentials.get(&id).cloned())
    }

    fn find_by_identity_id(&self, identity_id: IdentityId) -> Result<Vec<Credential>, Self::Error> {
        Ok(self
            .credentials
            .values()
            .filter(|item| item.identity_id() == identity_id)
            .cloned()
            .collect())
    }

    fn save(&mut self, credential: Credential) -> Result<(), Self::Error> {
        self.credentials.insert(credential.id(), credential);

        Ok(())
    }
}
