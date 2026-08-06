use localid_client::ClientId;
use localid_credential::CredentialId;
use localid_password::PasswordSecret;

/// Request to authenticate using a password Credential.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatePasswordRequest {
    client_id: ClientId,
    credential_id: CredentialId,
    password: PasswordSecret,
}

impl AuthenticatePasswordRequest {
    /// Creates a password authentication request.
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

    /// Returns the target Client identifier.
    #[must_use]
    pub const fn client_id(&self) -> ClientId {
        self.client_id
    }

    /// Returns the target Credential identifier.
    #[must_use]
    pub const fn credential_id(&self) -> CredentialId {
        self.credential_id
    }

    /// Returns the supplied password.
    #[must_use]
    pub const fn password(&self) -> &PasswordSecret {
        &self.password
    }
}
