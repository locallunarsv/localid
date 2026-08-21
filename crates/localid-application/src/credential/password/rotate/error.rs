use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// Errors produced while rotating a password Credential.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotatePasswordCredentialError {
    /// Credential was not found.
    CredentialNotFound,

    /// Credential is not a password Credential.
    InvalidCredentialKind,

    /// Credential has been revoked.
    CredentialRevoked,

    /// Password material was not found.
    PasswordMaterialNotFound,

    /// Credential repository operation failed.
    CredentialRepositoryFailure,

    /// Password material repository operation failed.
    PasswordMaterialRepositoryFailure,

    /// Password hashing failed.
    PasswordHashingFailure,
}

impl Display for RotatePasswordCredentialError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CredentialNotFound => formatter.write_str("Credential not found"),
            Self::InvalidCredentialKind => {
                formatter.write_str("Credential is not a password Credential")
            }
            Self::CredentialRevoked => formatter.write_str("Credential has been revoked"),
            Self::PasswordMaterialNotFound => formatter.write_str("Password material not found"),
            Self::CredentialRepositoryFailure => {
                formatter.write_str("Credential repository operation failed")
            }
            Self::PasswordMaterialRepositoryFailure => {
                formatter.write_str("Password material repository operation failed")
            }
            Self::PasswordHashingFailure => formatter.write_str("Password hashing failed"),
        }
    }
}

impl Error for RotatePasswordCredentialError {}
