use localid_password::PasswordSecret;

/// Evidence presented to prove possession of a Credential.
///
/// Each variant carries evidence specific to one authentication mechanism.
/// Sensitive values must remain redacted from debug output.
#[derive(Clone, PartialEq, Eq)]
pub enum AuthenticationEvidence {
    /// Plain-text password evidence.
    Password(PasswordSecret),
}

impl AuthenticationEvidence {
    /// Creates password authentication evidence.
    #[must_use]
    pub fn password(secret: PasswordSecret) -> Self {
        Self::Password(secret)
    }

    /// Returns the password secret when this is password evidence.
    #[must_use]
    pub const fn as_password(&self) -> Option<&PasswordSecret> {
        match self {
            Self::Password(secret) => Some(secret),
        }
    }
}

impl std::fmt::Debug for AuthenticationEvidence {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Password(_) => formatter.write_str("AuthenticationEvidence::Password(REDACTED)"),
        }
    }
}

#[cfg(test)]
mod tests {
    use localid_password::PasswordSecret;

    use super::AuthenticationEvidence;

    #[test]
    fn creates_password_evidence() {
        let secret =
            PasswordSecret::new("correct-password").expect("non-empty password should be accepted");

        let evidence = AuthenticationEvidence::password(secret.clone());

        assert_eq!(evidence.as_password(), Some(&secret));
    }

    #[test]
    fn debug_output_does_not_expose_password() {
        let evidence = AuthenticationEvidence::password(
            PasswordSecret::new("super-secret-password")
                .expect("non-empty password should be accepted"),
        );

        let output = format!("{evidence:?}");

        assert_eq!(output, "AuthenticationEvidence::Password(REDACTED)");
        assert!(!output.contains("super-secret-password"));
    }
}
