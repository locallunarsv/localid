use localid_oauth_client::OAuthClientRepository;

use super::{ListOAuthClientsError, ListOAuthClientsResult};

/// Lists OAuth clients use case.
pub struct ListOAuthClientsUseCase<R> {
    repository: R,
}

impl<R> ListOAuthClientsUseCase<R> {
    /// Creates a new list OAuth clients use case.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> ListOAuthClientsUseCase<R>
where
    R: OAuthClientRepository,
{
    /// Executes OAuth client listing.
    pub fn execute(&self) -> Result<ListOAuthClientsResult, ListOAuthClientsError> {
        let clients = self
            .repository
            .find_all()
            .map_err(|_| ListOAuthClientsError::RepositoryFailure)?;

        Ok(ListOAuthClientsResult::new(clients))
    }
}
