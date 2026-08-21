use localid_repository::IdentityRepository;

use super::{DisableIdentityCommand, DisableIdentityError};

/// Disables an identity use case.
pub struct DisableIdentityUseCase<R> {
    repository: R,
}

impl<R> DisableIdentityUseCase<R> {
    /// Creates a new disable identity use case.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> DisableIdentityUseCase<R>
where
    R: IdentityRepository,
{
    /// Executes identity disable operation.
    pub fn execute(&mut self, command: DisableIdentityCommand) -> Result<(), DisableIdentityError> {
        let mut identity = self
            .repository
            .find_by_id(command.identity_id())
            .map_err(|_| DisableIdentityError::RepositoryFailure)?
            .ok_or(DisableIdentityError::NotFound)?;

        identity
            .disable()
            .map_err(|_| DisableIdentityError::AlreadyDeleted)?;

        self.repository
            .save(identity)
            .map_err(|_| DisableIdentityError::RepositoryFailure)?;

        Ok(())
    }
}
