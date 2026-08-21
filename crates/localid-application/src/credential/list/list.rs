use localid_identity::IdentityId;
use localid_repository::CredentialRepository;

use super::{ListCredentialsError, ListCredentialsResult};

/// Lists Credentials owned by an Identity.
pub struct ListCredentialsUseCase<R> {
    repository: R,
}

impl<R> ListCredentialsUseCase<R> {
    /// Creates a new Credential listing use case.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> ListCredentialsUseCase<R>
where
    R: CredentialRepository,
{
    /// Executes Credential listing for an Identity.
    pub fn execute(
        &self,
        identity_id: IdentityId,
    ) -> Result<ListCredentialsResult, ListCredentialsError> {
        let credentials = self
            .repository
            .find_by_identity_id(identity_id)
            .map_err(|_| ListCredentialsError::RepositoryFailure)?;

        Ok(ListCredentialsResult::new(credentials))
    }
}
