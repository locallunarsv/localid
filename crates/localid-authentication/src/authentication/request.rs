use localid_credential::CredentialId;

use super::AuthenticationEvidence;

/// Request to authenticate a Credential.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticateRequest {
    credential_id: CredentialId,
    evidence: AuthenticationEvidence,
}

impl AuthenticateRequest {
    /// Creates a new authentication request.
    #[must_use]
    pub const fn new(credential_id: CredentialId, evidence: AuthenticationEvidence) -> Self {
        Self {
            credential_id,
            evidence,
        }
    }

    /// Returns the target Credential identifier.
    #[must_use]
    pub const fn credential_id(&self) -> CredentialId {
        self.credential_id
    }

    /// Returns the presented authentication evidence.
    #[must_use]
    pub const fn evidence(&self) -> &AuthenticationEvidence {
        &self.evidence
    }
}

#[cfg(test)]
mod tests {
    use super::AuthenticateRequest;
    use crate::AuthenticationEvidence;
    use localid_credential::CredentialId;
    use localid_password::PasswordSecret;

    #[test]
    fn creates_authentication_request() {
        let credential_id = CredentialId::new();
        let evidence = AuthenticationEvidence::password(
            PasswordSecret::new("test-password").expect("test password should be valid"),
        );

        let request = AuthenticateRequest::new(credential_id, evidence.clone());

        assert_eq!(request.credential_id(), credential_id);
        assert_eq!(request.evidence(), &evidence);
    }
}
