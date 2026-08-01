use localid_credential::CredentialId;
use localid_password::PasswordCredential;

/// Repository for password-specific Credential material.
pub trait PasswordCredentialRepository {
    /// Repository-specific error.
    type Error;

    /// Finds password material associated with a Credential.
    ///
    /// # Errors
    ///
    /// Returns the repository-specific error when the lookup cannot be
    /// completed.
    fn find_by_credential_id(
        &self,
        credential_id: CredentialId,
    ) -> Result<Option<PasswordCredential>, Self::Error>;

    /// Persists password material associated with a Credential.
    ///
    /// Existing material for the same Credential may be replaced.
    ///
    /// # Errors
    ///
    /// Returns the repository-specific error when persistence cannot be
    /// completed.
    fn save(&mut self, password: PasswordCredential) -> Result<(), Self::Error>;
}
