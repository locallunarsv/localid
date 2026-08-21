use localid_repository::IdentityRepository;

use super::{EnableIdentityCommand, EnableIdentityError};

/// Enables an identity use case.
pub struct EnableIdentityUseCase<R> {
    repository: R,
}

impl<R> EnableIdentityUseCase<R> {
    /// Creates a new enable identity use case.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> EnableIdentityUseCase<R>
where
    R: IdentityRepository,
{
    /// Executes identity enable operation.
    pub fn execute(&mut self, command: EnableIdentityCommand) -> Result<(), EnableIdentityError> {
        let mut identity = self
            .repository
            .find_by_id(command.identity_id())
            .map_err(|_| EnableIdentityError::RepositoryFailure)?
            .ok_or(EnableIdentityError::NotFound)?;

        identity
            .enable()
            .map_err(|_| EnableIdentityError::AlreadyDeleted)?;

        self.repository
            .save(identity)
            .map_err(|_| EnableIdentityError::RepositoryFailure)?;

        Ok(())
    }
}
