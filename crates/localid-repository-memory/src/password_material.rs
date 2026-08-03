use std::collections::HashMap;

use localid_credential::CredentialId;
use localid_password::PasswordMaterial;
use localid_repository::PasswordMaterialRepository;

use crate::MemoryRepositoryError;

/// In-memory password material repository.
#[derive(Debug, Default)]
pub struct MemoryPasswordMaterialRepository {
    materials: HashMap<CredentialId, PasswordMaterial>,
}

impl MemoryPasswordMaterialRepository {
    /// Creates empty repository.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl PasswordMaterialRepository for MemoryPasswordMaterialRepository {
    type Error = MemoryRepositoryError;

    fn find_by_credential_id(
        &self,
        credential_id: CredentialId,
    ) -> Result<Option<PasswordMaterial>, Self::Error> {
        Ok(self.materials.get(&credential_id).cloned())
    }

    fn save(&mut self, material: PasswordMaterial) -> Result<(), Self::Error> {
        self.materials.insert(material.credential_id(), material);

        Ok(())
    }
}
