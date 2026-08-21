use localid_repository::CredentialRepository;

use super::{RevokeCredentialCommand, RevokeCredentialError};

/// Revokes a Credential use case.
pub struct RevokeCredentialUseCase<R> {
    repository: R,
}

impl<R> RevokeCredentialUseCase<R> {
    /// Creates a new revoke Credential use case.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> RevokeCredentialUseCase<R>
where
    R: CredentialRepository,
{
    /// Executes Credential revoke operation.
    pub fn execute(
        &mut self,
        command: RevokeCredentialCommand,
    ) -> Result<(), RevokeCredentialError> {
        let mut credential = self
            .repository
            .find_by_id(command.credential_id())
            .map_err(|_| RevokeCredentialError::RepositoryFailure)?
            .ok_or(RevokeCredentialError::NotFound)?;

        credential.revoke();

        self.repository
            .save(credential)
            .map_err(|_| RevokeCredentialError::RepositoryFailure)?;

        Ok(())
    }
}
