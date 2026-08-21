use localid_credential::CredentialId;
use localid_repository::CredentialRepository;

use super::{GetCredentialError, GetCredentialResult};

/// Gets a Credential by identifier.
pub struct GetCredentialUseCase<R> {
    repository: R,
}

impl<R> GetCredentialUseCase<R> {
    /// Creates a new Credential lookup use case.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> GetCredentialUseCase<R>
where
    R: CredentialRepository,
{
    /// Executes Credential lookup.
    pub fn execute(
        &self,
        credential_id: CredentialId,
    ) -> Result<GetCredentialResult, GetCredentialError> {
        let credential = self
            .repository
            .find_by_id(credential_id)
            .map_err(|_| GetCredentialError::RepositoryFailure)?
            .ok_or(GetCredentialError::NotFound)?;

        Ok(GetCredentialResult::new(credential))
    }
}
