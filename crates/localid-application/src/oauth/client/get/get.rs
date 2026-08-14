use localid_oauth_client::OAuthClientRepository;

use super::{GetOAuthClientError, GetOAuthClientQuery, GetOAuthClientResult};

/// Gets OAuth client use case.
pub struct GetOAuthClientUseCase<R> {
    repository: R,
}

impl<R> GetOAuthClientUseCase<R> {
    /// Creates a new use case.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> GetOAuthClientUseCase<R>
where
    R: OAuthClientRepository,
{
    /// Executes OAuth client lookup.
    pub fn execute(
        &self,
        query: GetOAuthClientQuery,
    ) -> Result<GetOAuthClientResult, GetOAuthClientError> {
        let client = self
            .repository
            .find_by_id(query.client_id())
            .map_err(|_| GetOAuthClientError::RepositoryFailure)?
            .ok_or(GetOAuthClientError::NotFound)?;

        Ok(GetOAuthClientResult::new(client))
    }
}
