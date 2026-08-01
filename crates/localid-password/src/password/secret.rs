use std::fmt::{Debug, Formatter};

/// Plain-text password presented during authentication.
///
/// A password secret should exist only temporarily in memory and must never be
/// persisted or written to logs.
#[derive(Clone, PartialEq, Eq)]
pub struct PasswordSecret(String);

impl PasswordSecret {
    /// Creates a password secret from its plain-text representation.
    #[must_use]
    pub fn new(secret: String) -> Self {
        Self(secret)
    }

    /// Returns the plain-text password.
    ///
    /// Callers must avoid logging or persisting the returned value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Debug for PasswordSecret {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("PasswordSecret(REDACTED)")
    }
}

#[cfg(test)]
mod tests {
    use super::PasswordSecret;

    #[test]
    fn stores_password_secret() {
        let secret = PasswordSecret::new("correct horse battery staple".to_owned());

        assert_eq!(secret.as_str(), "correct horse battery staple");
    }

    #[test]
    fn debug_output_does_not_expose_password() {
        let secret = PasswordSecret::new("super-secret-password".to_owned());

        let output = format!("{secret:?}");

        assert_eq!(output, "PasswordSecret(REDACTED)");
        assert!(!output.contains("super-secret-password"));
    }

    #[test]
    fn password_secrets_are_comparable() {
        let first = PasswordSecret::new("same-password".to_owned());
        let second = PasswordSecret::new("same-password".to_owned());

        assert_eq!(first, second);
    }
}
