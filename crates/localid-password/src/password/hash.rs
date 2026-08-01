use std::fmt::{Debug, Display, Formatter};

/// Stored password hash.
///
/// The value may contain an encoded password-hashing representation including
/// algorithm parameters and salt information.
#[derive(Clone, PartialEq, Eq)]
pub struct PasswordHash(String);

impl PasswordHash {
    /// Creates a password hash from its encoded representation.
    #[must_use]
    pub fn new(hash: String) -> Self {
        Self(hash)
    }

    /// Returns the encoded password hash.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the value and returns the encoded password hash.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl Debug for PasswordHash {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("PasswordHash(REDACTED)")
    }
}

impl Display for PasswordHash {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::PasswordHash;

    #[test]
    fn stores_encoded_password_hash() {
        let hash = PasswordHash::new("$example$encoded-password-hash".to_owned());

        assert_eq!(hash.as_str(), "$example$encoded-password-hash");
    }

    #[test]
    fn converts_password_hash_into_string() {
        let hash = PasswordHash::new("$example$encoded-password-hash".to_owned());

        assert_eq!(hash.into_string(), "$example$encoded-password-hash");
    }

    #[test]
    fn debug_output_does_not_expose_hash() {
        let hash = PasswordHash::new("$example$sensitive-hash".to_owned());

        let output = format!("{hash:?}");

        assert_eq!(output, "PasswordHash(REDACTED)");
        assert!(!output.contains("$example$sensitive-hash"));
    }

    #[test]
    fn display_returns_encoded_hash() {
        let hash = PasswordHash::new("$example$encoded-password-hash".to_owned());

        assert_eq!(hash.to_string(), "$example$encoded-password-hash");
    }
}
