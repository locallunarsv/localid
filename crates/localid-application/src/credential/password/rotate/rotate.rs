use localid_credential::{CredentialKind, CredentialLifecycleState};
use localid_password::PasswordHasher;
use localid_repository::{CredentialRepository, PasswordMaterialRepository};

use super::{RotatePasswordCredentialCommand, RotatePasswordCredentialError};

/// Rotates password material associated with a password Credential.
pub struct RotatePasswordCredentialUseCase<CR, PR, H> {
    credential_repository: CR,
    password_material_repository: PR,
    password_hasher: H,
}

impl<CR, PR, H> RotatePasswordCredentialUseCase<CR, PR, H> {
    /// Creates a new password Credential rotation use case.
    #[must_use]
    pub const fn new(
        credential_repository: CR,
        password_material_repository: PR,
        password_hasher: H,
    ) -> Self {
        Self {
            credential_repository,
            password_material_repository,
            password_hasher,
        }
    }
}

impl<CR, PR, H> RotatePasswordCredentialUseCase<CR, PR, H>
where
    CR: CredentialRepository,
    PR: PasswordMaterialRepository,
    H: PasswordHasher,
{
    /// Executes password Credential rotation.
    pub fn execute(
        &mut self,
        command: RotatePasswordCredentialCommand,
    ) -> Result<(), RotatePasswordCredentialError> {
        let credential = self
            .credential_repository
            .find_by_id(command.credential_id())
            .map_err(|_| RotatePasswordCredentialError::CredentialRepositoryFailure)?
            .ok_or(RotatePasswordCredentialError::CredentialNotFound)?;

        if credential.kind() != CredentialKind::Password {
            return Err(RotatePasswordCredentialError::InvalidCredentialKind);
        }

        if credential.lifecycle_state() == CredentialLifecycleState::Revoked {
            return Err(RotatePasswordCredentialError::CredentialRevoked);
        }

        let mut material = self
            .password_material_repository
            .find_by_credential_id(command.credential_id())
            .map_err(|_| RotatePasswordCredentialError::PasswordMaterialRepositoryFailure)?
            .ok_or(RotatePasswordCredentialError::PasswordMaterialNotFound)?;

        let password_hash = self
            .password_hasher
            .hash(command.password())
            .map_err(|_| RotatePasswordCredentialError::PasswordHashingFailure)?;

        material.replace_hash(password_hash);

        self.password_material_repository
            .save(material)
            .map_err(|_| RotatePasswordCredentialError::PasswordMaterialRepositoryFailure)?;

        Ok(())
    }
}
