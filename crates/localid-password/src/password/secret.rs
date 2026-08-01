use std::fmt::{Debug, Formatter};

use super::PasswordError;

/// Plain-text password presented during authentication.
///
/// A password secret should exist only temporarily in memory and must never be
/// persisted or written to logs.
#[derive(Clone, PartialEq, Eq)]
pub struct PasswordSecret(String);

impl PasswordSecret {
    /// Creates a password secret from its plain-text representation.
    ///
    /// # Errors
    ///
    /// Returns [`PasswordError::EmptySecret`] when the supplied password is
    /// empty.
    pub fn new(secret: impl Into<String>) -> Result<Self, PasswordError> {
        let secret = secret.into();

        if secret.is_empty() {
            return Err(PasswordError::EmptySecret);
        }

        Ok(Self(secret))
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
    use crate::PasswordError;

    #[test]
    fn creates_password_secret() {
        let secret = PasswordSecret::new("correct horse battery staple")
            .expect("non-empty password should be accepted");

        assert_eq!(secret.as_str(), "correct horse battery staple");
    }

    #[test]
    fn rejects_empty_password_secret() {
        let result = PasswordSecret::new("");

        assert_eq!(result, Err(PasswordError::EmptySecret));
    }

    #[test]
    fn debug_output_does_not_expose_password() {
        let secret = PasswordSecret::new("super-secret-password")
            .expect("non-empty password should be accepted");

        let output = format!("{secret:?}");

        assert_eq!(output, "PasswordSecret(REDACTED)");
        assert!(!output.contains("super-secret-password"));
    }

    #[test]
    fn password_secrets_are_comparable() {
        let first = PasswordSecret::new("same-password").unwrap();
        let second = PasswordSecret::new("same-password").unwrap();

        assert_eq!(first, second);
    }
}
