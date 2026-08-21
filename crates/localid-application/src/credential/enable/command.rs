use localid_credential::CredentialId;

/// Command for enabling a Credential.
#[derive(Debug, Clone, Copy)]
pub struct EnableCredentialCommand {
    credential_id: CredentialId,
}

impl EnableCredentialCommand {
    /// Creates a new enable Credential command.
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
