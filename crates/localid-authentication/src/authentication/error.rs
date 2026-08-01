use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// Errors that may occur during authentication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthenticationError {
    /// The requested Credential could not be found.
    CredentialNotFound,

    /// The Credential cannot currently be used.
    CredentialUnavailable,

    /// The owning Identity could not be found.
    IdentityNotFound,

    /// The Identity cannot currently be used.
    IdentityUnavailable,

    /// Authentication evidence did not match the Credential.
    InvalidEvidence,

    /// The Credential repository could not complete its operation.
    CredentialRepositoryFailure,

    /// The Identity repository could not complete its operation.
    IdentityRepositoryFailure,

    /// The Session repository could not complete its operation.
    SessionRepositoryFailure,

    /// Credential verification could not be completed.
    VerificationFailure,

    /// A Session could not be created.
    SessionCreationFailure,

    /// The requested Credential is not a password Credential.
    InvalidCredentialKind,

    /// Password material associated with the Credential could not be found.
    PasswordMaterialNotFound,

    /// The Password Material repository could not complete its operation.
    PasswordMaterialRepositoryFailure,
}

impl Display for AuthenticationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::CredentialNotFound => "credential not found",
            Self::CredentialUnavailable => "credential is unavailable",
            Self::IdentityNotFound => "identity not found",
            Self::IdentityUnavailable => "identity is unavailable",
            Self::InvalidEvidence => "authentication evidence is invalid",
            Self::CredentialRepositoryFailure => "credential repository operation failed",
            Self::IdentityRepositoryFailure => "identity repository operation failed",
            Self::SessionRepositoryFailure => "session repository operation failed",
            Self::VerificationFailure => "credential verification could not be completed",
            Self::SessionCreationFailure => "session could not be created",
            Self::InvalidCredentialKind => {
                "credential kind is incompatible with password authentication"
            }
            Self::PasswordMaterialNotFound => "password material not found",
            Self::PasswordMaterialRepositoryFailure => {
                "password material repository operation failed"
            }
        };

        formatter.write_str(message)
    }
}

impl Error for AuthenticationError {}

#[cfg(test)]
mod tests {
    use super::AuthenticationError;

    #[test]
    fn authentication_errors_have_stable_messages() {
        let cases = [
            (
                AuthenticationError::CredentialNotFound,
                "credential not found",
            ),
            (
                AuthenticationError::CredentialUnavailable,
                "credential is unavailable",
            ),
            (AuthenticationError::IdentityNotFound, "identity not found"),
            (
                AuthenticationError::IdentityUnavailable,
                "identity is unavailable",
            ),
            (
                AuthenticationError::InvalidEvidence,
                "authentication evidence is invalid",
            ),
            (
                AuthenticationError::CredentialRepositoryFailure,
                "credential repository operation failed",
            ),
            (
                AuthenticationError::IdentityRepositoryFailure,
                "identity repository operation failed",
            ),
            (
                AuthenticationError::SessionRepositoryFailure,
                "session repository operation failed",
            ),
            (
                AuthenticationError::VerificationFailure,
                "credential verification could not be completed",
            ),
            (
                AuthenticationError::SessionCreationFailure,
                "session could not be created",
            ),
            (
                AuthenticationError::InvalidCredentialKind,
                "credential kind is incompatible with password authentication",
            ),
            (
                AuthenticationError::PasswordMaterialNotFound,
                "password material not found",
            ),
            (
                AuthenticationError::PasswordMaterialRepositoryFailure,
                "password material repository operation failed",
            ),
        ];

        for (error, expected) in cases {
            assert_eq!(error.to_string(), expected);
        }
    }
}
