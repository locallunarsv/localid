use localid_oauth_client::OAuthClientRepository;

use super::{ActivateOAuthClientCommand, ActivateOAuthClientError};

/// Activates OAuth client use case.
pub struct ActivateOAuthClientUseCase<R> {
    repository: R,
}

impl<R> ActivateOAuthClientUseCase<R> {
    /// Creates a new use case.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> ActivateOAuthClientUseCase<R>
where
    R: OAuthClientRepository,
{
    /// Executes OAuth client activation.
    pub fn execute(
        &mut self,
        command: ActivateOAuthClientCommand,
    ) -> Result<(), ActivateOAuthClientError> {
        let mut client = self
            .repository
            .find_by_id(command.client_id())
            .map_err(|_| ActivateOAuthClientError::RepositoryFailure)?
            .ok_or(ActivateOAuthClientError::NotFound)?;

        client
            .activate()
            .map_err(|_| ActivateOAuthClientError::AlreadyDeleted)?;

        self.repository
            .save(client)
            .map_err(|_| ActivateOAuthClientError::RepositoryFailure)?;

        Ok(())
    }
}
