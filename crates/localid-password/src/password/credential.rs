use localid_credential::CredentialId;

use super::PasswordHash;

/// Password-specific authentication material associated with a Credential.
///
/// `PasswordCredential` stores only password-specific data. Ownership,
/// Credential kind, and lifecycle remain managed by the primary Credential
/// aggregate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordCredential {
    credential_id: CredentialId,
    password_hash: PasswordHash,
}

impl PasswordCredential {
    /// Creates password material for an existing Credential.
    #[must_use]
    pub const fn new(credential_id: CredentialId, password_hash: PasswordHash) -> Self {
        Self {
            credential_id,
            password_hash,
        }
    }

    /// Returns the associated Credential identifier.
    #[must_use]
    pub const fn credential_id(&self) -> CredentialId {
        self.credential_id
    }

    /// Returns the stored password hash.
    #[must_use]
    pub const fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }

    /// Replaces the stored password hash.
    ///
    /// Password policy and hashing remain responsibilities of callers and
    /// password infrastructure.
    pub fn replace_hash(&mut self, password_hash: PasswordHash) {
        self.password_hash = password_hash;
    }
}

#[cfg(test)]
mod tests {
    use localid_credential::CredentialId;

    use super::PasswordCredential;
    use crate::PasswordHash;

    #[test]
    fn creates_password_credential() {
        let credential_id = CredentialId::new();
        let password_hash = PasswordHash::new("$example$hash".to_owned());

        let password = PasswordCredential::new(credential_id, password_hash.clone());

        assert_eq!(password.credential_id(), credential_id);
        assert_eq!(password.password_hash(), &password_hash);
    }

    #[test]
    fn replaces_password_hash() {
        let mut password = PasswordCredential::new(
            CredentialId::new(),
            PasswordHash::new("$example$old".to_owned()),
        );

        let new_hash = PasswordHash::new("$example$new".to_owned());

        password.replace_hash(new_hash.clone());

        assert_eq!(password.password_hash(), &new_hash);
    }
}
