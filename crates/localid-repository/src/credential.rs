use localid_credential::{Credential, CredentialId};
use localid_identity::IdentityId;

/// Repository for Credential aggregates.
pub trait CredentialRepository {
    /// Repository-specific error.
    type Error;

    /// Finds a Credential by its identifier.
    fn find_by_id(&self, id: CredentialId) -> Result<Option<Credential>, Self::Error>;

    /// Finds all Credentials owned by an Identity.
    fn find_by_identity_id(&self, identity_id: IdentityId) -> Result<Vec<Credential>, Self::Error>;

    /// Persists a Credential.
    fn save(&mut self, credential: Credential) -> Result<(), Self::Error>;
}
