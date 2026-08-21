use localid_repository::CredentialRepository;

use super::{DisableCredentialCommand, DisableCredentialError};

/// Disables a Credential use case.
pub struct DisableCredentialUseCase<R> {
    repository: R,
}

impl<R> DisableCredentialUseCase<R> {
    /// Creates a new disable Credential use case.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> DisableCredentialUseCase<R>
where
    R: CredentialRepository,
{
    /// Executes Credential disable operation.
    pub fn execute(
        &mut self,
        command: DisableCredentialCommand,
    ) -> Result<(), DisableCredentialError> {
        let mut credential = self
            .repository
            .find_by_id(command.credential_id())
            .map_err(|_| DisableCredentialError::RepositoryFailure)?
            .ok_or(DisableCredentialError::NotFound)?;

        credential
            .disable()
            .map_err(|_| DisableCredentialError::AlreadyRevoked)?;

        self.repository
            .save(credential)
            .map_err(|_| DisableCredentialError::RepositoryFailure)?;

        Ok(())
    }
}
