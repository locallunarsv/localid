use localid_oauth_client::OAuthClientRepository;

use super::{DeleteOAuthClientCommand, DeleteOAuthClientError};

/// Deletes OAuth client use case.
pub struct DeleteOAuthClientUseCase<R> {
    repository: R,
}

impl<R> DeleteOAuthClientUseCase<R> {
    /// Creates a new use case.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> DeleteOAuthClientUseCase<R>
where
    R: OAuthClientRepository,
{
    /// Executes OAuth client deletion.
    pub fn execute(
        &mut self,
        command: DeleteOAuthClientCommand,
    ) -> Result<(), DeleteOAuthClientError> {
        let mut client = self
            .repository
            .find_by_id(command.client_id())
            .map_err(|_| DeleteOAuthClientError::RepositoryFailure)?
            .ok_or(DeleteOAuthClientError::NotFound)?;

        client
            .delete()
            .map_err(|_| DeleteOAuthClientError::AlreadyDeleted)?;

        self.repository
            .save(client)
            .map_err(|_| DeleteOAuthClientError::RepositoryFailure)?;

        Ok(())
    }
}
