use localid_identity::IdentityId;
use localid_password::PasswordSecret;

/// Command for creating a password credential.
#[derive(Debug)]
pub struct CreatePasswordCredentialCommand {
    identity_id: IdentityId,
    password: PasswordSecret,
}

impl CreatePasswordCredentialCommand {
    /// Creates a new password credential command.
    #[must_use]
    pub const fn new(identity_id: IdentityId, password: PasswordSecret) -> Self {
        Self {
            identity_id,
            password,
        }
    }

    /// Returns the owning Identity identifier.
    #[must_use]
    pub const fn identity_id(&self) -> IdentityId {
        self.identity_id
    }

    /// Returns the password secret.
    #[must_use]
    pub const fn password(&self) -> &PasswordSecret {
        &self.password
    }
}
