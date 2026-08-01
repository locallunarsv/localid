use localid_credential::CredentialId;
use localid_password::PasswordSecret;

/// Request to authenticate using a password Credential.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatePasswordRequest {
    credential_id: CredentialId,
    password: PasswordSecret,
}

impl AuthenticatePasswordRequest {
    /// Creates a password authentication request.
    #[must_use]
    pub const fn new(credential_id: CredentialId, password: PasswordSecret) -> Self {
        Self {
            credential_id,
            password,
        }
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

#[cfg(test)]
mod tests {
    use localid_credential::CredentialId;
    use localid_password::PasswordSecret;

    use super::AuthenticatePasswordRequest;

    #[test]
    fn creates_password_authentication_request() {
        let credential_id = CredentialId::new();
        let password = PasswordSecret::new("test-password").expect("test password should be valid");

        let request = AuthenticatePasswordRequest::new(credential_id, password.clone());

        assert_eq!(request.credential_id(), credential_id);
        assert_eq!(request.password(), &password);
    }
}
