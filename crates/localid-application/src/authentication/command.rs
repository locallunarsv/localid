use localid_credential::CredentialId;
use localid_password::PasswordSecret;

/// Login authentication request.
#[derive(Debug)]
pub struct LoginCommand {
    credential_id: CredentialId,
    password: PasswordSecret,
}

impl LoginCommand {
    /// Creates a new login command.
    #[must_use]
    pub const fn new(credential_id: CredentialId, password: PasswordSecret) -> Self {
        Self {
            credential_id,
            password,
        }
    }

    /// Returns credential identifier.
    #[must_use]
    pub const fn credential_id(&self) -> CredentialId {
        self.credential_id
    }

    /// Returns password secret.
    #[must_use]
    pub fn password(&self) -> &PasswordSecret {
        &self.password
    }
}
