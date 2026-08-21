use localid_credential::{Credential, CredentialId, CredentialKind};
use localid_password::{PasswordHasher, PasswordMaterial};
use localid_repository::{CredentialRepository, IdentityRepository, PasswordMaterialRepository};

use super::{
    CreatePasswordCredentialCommand, CreatePasswordCredentialError, CreatePasswordCredentialResult,
};

/// Creates a password credential use case.
pub struct CreatePasswordCredentialUseCase<IR, CR, PR, H> {
    identity_repository: IR,
    credential_repository: CR,
    password_material_repository: PR,
    password_hasher: H,
}

impl<IR, CR, PR, H> CreatePasswordCredentialUseCase<IR, CR, PR, H> {
    /// Creates a new password credential use case.
    #[must_use]
    pub const fn new(
        identity_repository: IR,
        credential_repository: CR,
        password_material_repository: PR,
        password_hasher: H,
    ) -> Self {
        Self {
            identity_repository,
            credential_repository,
            password_material_repository,
            password_hasher,
        }
    }
}

impl<IR, CR, PR, H> CreatePasswordCredentialUseCase<IR, CR, PR, H>
where
    IR: IdentityRepository,
    CR: CredentialRepository,
    PR: PasswordMaterialRepository,
    H: PasswordHasher,
{
    /// Executes password credential creation.
    pub fn execute(
        &mut self,
        command: CreatePasswordCredentialCommand,
    ) -> Result<CreatePasswordCredentialResult, CreatePasswordCredentialError> {
        let identity = self
            .identity_repository
            .find_by_id(command.identity_id())
            .map_err(|_| CreatePasswordCredentialError::IdentityRepositoryFailure)?
            .ok_or(CreatePasswordCredentialError::IdentityNotFound)?;

        if !identity.is_active() {
            return Err(CreatePasswordCredentialError::IdentityNotActive);
        }

        let existing_credentials = self
            .credential_repository
            .find_by_identity_id(command.identity_id())
            .map_err(|_| CreatePasswordCredentialError::CredentialRepositoryFailure)?;

        let password_credential_exists = existing_credentials.iter().any(|credential| {
            credential.kind() == CredentialKind::Password && !credential.is_revoked()
        });

        if password_credential_exists {
            return Err(CreatePasswordCredentialError::PasswordCredentialAlreadyExists);
        }

        let credential_id = CredentialId::new();

        let password_hash = self
            .password_hasher
            .hash(command.password())
            .map_err(|_| CreatePasswordCredentialError::PasswordHashingFailure)?;

        let credential = Credential::new(
            credential_id,
            command.identity_id(),
            CredentialKind::Password,
        );

        let password_material = PasswordMaterial::new(credential_id, password_hash);

        self.credential_repository
            .save(credential)
            .map_err(|_| CreatePasswordCredentialError::CredentialRepositoryFailure)?;

        self.password_material_repository
            .save(password_material)
            .map_err(|_| CreatePasswordCredentialError::PasswordMaterialRepositoryFailure)?;

        Ok(CreatePasswordCredentialResult::new(credential_id))
    }
}
