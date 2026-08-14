use localid_oauth_client::OAuthClientRepository;

use super::{DisableOAuthClientCommand, DisableOAuthClientError};

/// Disables OAuth client use case.
pub struct DisableOAuthClientUseCase<R> {
    repository: R,
}

impl<R> DisableOAuthClientUseCase<R> {
    /// Creates a new use case.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> DisableOAuthClientUseCase<R>
where
    R: OAuthClientRepository,
{
    /// Executes OAuth client disable operation.
    pub fn execute(
        &mut self,
        command: DisableOAuthClientCommand,
    ) -> Result<(), DisableOAuthClientError> {
        let mut client = self
            .repository
            .find_by_id(command.client_id())
            .map_err(|_| DisableOAuthClientError::RepositoryFailure)?
            .ok_or(DisableOAuthClientError::NotFound)?;

        client
            .disable()
            .map_err(|_| DisableOAuthClientError::AlreadyDeleted)?;

        self.repository
            .save(client)
            .map_err(|_| DisableOAuthClientError::RepositoryFailure)?;

        Ok(())
    }
}
