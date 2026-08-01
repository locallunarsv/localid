use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// Errors that may occur during authentication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthenticationError {
    /// The requested Credential could not be found.
    CredentialNotFound,

    /// The owning Identity could not be found.
    IdentityNotFound,

    /// Authentication evidence did not match the Credential.
    InvalidEvidence,
}

impl Display for AuthenticationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CredentialNotFound => formatter.write_str("credential not found"),
            Self::IdentityNotFound => formatter.write_str("identity not found"),
            Self::InvalidEvidence => formatter.write_str("authentication evidence is invalid"),
        }
    }
}

impl Error for AuthenticationError {}

#[cfg(test)]
mod tests {
    use super::AuthenticationError;

    #[test]
    fn credential_not_found_message() {
        assert_eq!(
            AuthenticationError::CredentialNotFound.to_string(),
            "credential not found"
        );
    }

    #[test]
    fn identity_not_found_message() {
        assert_eq!(
            AuthenticationError::IdentityNotFound.to_string(),
            "identity not found"
        );
    }

    #[test]
    fn invalid_evidence_message() {
        assert_eq!(
            AuthenticationError::InvalidEvidence.to_string(),
            "authentication evidence is invalid"
        );
    }
}
