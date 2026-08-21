use localid_repository::CredentialRepository;

use super::{EnableCredentialCommand, EnableCredentialError};

/// Enables a Credential use case.
pub struct EnableCredentialUseCase<R> {
    repository: R,
}

impl<R> EnableCredentialUseCase<R> {
    /// Creates a new enable Credential use case.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> EnableCredentialUseCase<R>
where
    R: CredentialRepository,
{
    /// Executes Credential enable operation.
    pub fn execute(
        &mut self,
        command: EnableCredentialCommand,
    ) -> Result<(), EnableCredentialError> {
        let mut credential = self
            .repository
            .find_by_id(command.credential_id())
            .map_err(|_| EnableCredentialError::RepositoryFailure)?
            .ok_or(EnableCredentialError::NotFound)?;

        credential
            .enable()
            .map_err(|_| EnableCredentialError::AlreadyRevoked)?;

        self.repository
            .save(credential)
            .map_err(|_| EnableCredentialError::RepositoryFailure)?;

        Ok(())
    }
}
