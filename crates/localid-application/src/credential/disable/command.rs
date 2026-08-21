use localid_credential::CredentialId;

/// Command for disabling a Credential.
#[derive(Debug, Clone, Copy)]
pub struct DisableCredentialCommand {
    credential_id: CredentialId,
}

impl DisableCredentialCommand {
    /// Creates a new disable Credential command.
    #[must_use]
    pub const fn new(credential_id: CredentialId) -> Self {
        Self { credential_id }
    }

    /// Returns the Credential identifier.
    #[must_use]
    pub const fn credential_id(&self) -> CredentialId {
        self.credential_id
    }
}
