use localid_credential::CredentialId;
use localid_password::PasswordMaterial;

/// Repository for password-specific authentication material.
pub trait PasswordMaterialRepository {
    /// Repository-specific error.
    type Error;

    /// Finds password material associated with a Credential.
    ///
    /// # Errors
    ///
    /// Returns the repository-specific error when lookup cannot be completed.
    fn find_by_credential_id(
        &self,
        credential_id: CredentialId,
    ) -> Result<Option<PasswordMaterial>, Self::Error>;

    /// Persists password material associated with a Credential.
    ///
    /// Existing material associated with the same Credential may be replaced.
    ///
    /// # Errors
    ///
    /// Returns the repository-specific error when persistence cannot be
    /// completed.
    fn save(&mut self, material: PasswordMaterial) -> Result<(), Self::Error>;
}
