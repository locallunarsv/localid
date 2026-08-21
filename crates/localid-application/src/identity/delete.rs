use localid_repository::IdentityRepository;

use super::{DeleteIdentityCommand, DeleteIdentityError};

/// Deletes an identity use case.
pub struct DeleteIdentityUseCase<R> {
    repository: R,
}

impl<R> DeleteIdentityUseCase<R> {
    /// Creates a new delete identity use case.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> DeleteIdentityUseCase<R>
where
    R: IdentityRepository,
{
    /// Executes identity delete operation.
    pub fn execute(&mut self, command: DeleteIdentityCommand) -> Result<(), DeleteIdentityError> {
        let mut identity = self
            .repository
            .find_by_id(command.identity_id())
            .map_err(|_| DeleteIdentityError::RepositoryFailure)?
            .ok_or(DeleteIdentityError::NotFound)?;

        identity.delete();

        self.repository
            .save(identity)
            .map_err(|_| DeleteIdentityError::RepositoryFailure)?;

        Ok(())
    }
}
