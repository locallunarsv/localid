use localid_credential::CredentialId;

use super::PasswordHash;

/// Password-specific authentication material associated with a Credential.
///
/// Ownership, Credential kind, and lifecycle remain managed by the primary
/// Credential aggregate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordMaterial {
    credential_id: CredentialId,
    password_hash: PasswordHash,
}

impl PasswordMaterial {
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
    pub fn replace_hash(&mut self, password_hash: PasswordHash) {
        self.password_hash = password_hash;
    }
}

#[cfg(test)]
mod tests {
    use localid_credential::CredentialId;

    use super::PasswordMaterial;
    use crate::PasswordHash;

    #[test]
    fn creates_password_material() {
        let credential_id = CredentialId::new();
        let password_hash = PasswordHash::new("$example$hash".to_owned());

        let material = PasswordMaterial::new(credential_id, password_hash.clone());

        assert_eq!(material.credential_id(), credential_id);
        assert_eq!(material.password_hash(), &password_hash);
    }

    #[test]
    fn replaces_password_hash() {
        let mut material = PasswordMaterial::new(
            CredentialId::new(),
            PasswordHash::new("$example$old".to_owned()),
        );

        let new_hash = PasswordHash::new("$example$new".to_owned());

        material.replace_hash(new_hash.clone());

        assert_eq!(material.password_hash(), &new_hash);
    }
}
