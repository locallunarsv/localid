use localid_credential::Credential;

/// Result returned after getting a Credential.
#[derive(Debug, Clone)]
pub struct GetCredentialResult {
    credential: Credential,
}

impl GetCredentialResult {
    /// Creates a new Credential result.
    #[must_use]
    pub const fn new(credential: Credential) -> Self {
        Self { credential }
    }

    /// Returns the Credential.
    #[must_use]
    pub const fn credential(&self) -> &Credential {
        &self.credential
    }
}
