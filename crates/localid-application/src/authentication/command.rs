use localid_client::ClientId;
use localid_credential::CredentialId;
use localid_password::PasswordSecret;

/// Login authentication request.
#[derive(Debug)]
pub struct LoginCommand {
    client_id: ClientId,
    credential_id: CredentialId,
    password: PasswordSecret,
}

impl LoginCommand {
    /// Creates a new login command.
    #[must_use]
    pub const fn new(
        client_id: ClientId,
        credential_id: CredentialId,
        password: PasswordSecret,
    ) -> Self {
        Self {
            client_id,
            credential_id,
            password,
        }
    }

    /// Returns client identifier.
    #[must_use]
    pub const fn client_id(&self) -> ClientId {
        self.client_id
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
