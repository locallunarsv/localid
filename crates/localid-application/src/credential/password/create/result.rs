use localid_credential::CredentialId;

/// Result returned after creating a password credential.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreatePasswordCredentialResult {
    credential_id: CredentialId,
}

impl CreatePasswordCredentialResult {
    /// Creates a new password credential result.
    #[must_use]
    pub const fn new(credential_id: CredentialId) -> Self {
        Self { credential_id }
    }

    /// Returns the created Credential identifier.
    #[must_use]
    pub const fn credential_id(&self) -> CredentialId {
        self.credential_id
    }
}
