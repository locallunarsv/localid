/// Authentication mechanism represented by a [`Credential`](crate::Credential).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CredentialKind {
    /// A human-memorable secret represented securely by a password-derived
    /// verifier.
    Password,

    /// A public-key Credential managed through a passkey authenticator.
    Passkey,

    /// A secret key intended for programmatic authentication.
    ApiKey,
}

impl CredentialKind {
    /// Returns `true` when this is a password Credential.
    #[must_use]
    pub const fn is_password(self) -> bool {
        matches!(self, Self::Password)
    }

    /// Returns `true` when this is a passkey Credential.
    #[must_use]
    pub const fn is_passkey(self) -> bool {
        matches!(self, Self::Passkey)
    }

    /// Returns `true` when this is an API-key Credential.
    #[must_use]
    pub const fn is_api_key(self) -> bool {
        matches!(self, Self::ApiKey)
    }
}

#[cfg(test)]
mod tests {
    use super::CredentialKind;

    #[test]
    fn credential_kind_predicates_are_correct() {
        assert!(CredentialKind::Password.is_password());
        assert!(CredentialKind::Passkey.is_passkey());
        assert!(CredentialKind::ApiKey.is_api_key());

        assert!(!CredentialKind::Password.is_passkey());
        assert!(!CredentialKind::Passkey.is_api_key());
        assert!(!CredentialKind::ApiKey.is_password());
    }
}
