use localid_credential::CredentialId;
use localid_password::PasswordSecret;

/// Command for rotating a password Credential.
pub struct RotatePasswordCredentialCommand {
    credential_id: CredentialId,
    password: PasswordSecret,
}

impl RotatePasswordCredentialCommand {
    /// Creates a new password Credential rotation command.
    #[must_use]
    pub const fn new(credential_id: CredentialId, password: PasswordSecret) -> Self {
        Self {
            credential_id,
            password,
        }
    }

    /// Returns the Credential identifier.
    #[must_use]
    pub const fn credential_id(&self) -> CredentialId {
        self.credential_id
    }

    /// Returns the new password secret.
    #[must_use]
    pub const fn password(&self) -> &PasswordSecret {
        &self.password
    }
}
