use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// Errors produced while enabling a Credential.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnableCredentialError {
    /// Credential was not found.
    NotFound,

    /// Credential has already been revoked.
    AlreadyRevoked,

    /// Credential repository operation failed.
    RepositoryFailure,
}

impl Display for EnableCredentialError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => formatter.write_str("Credential not found"),
            Self::AlreadyRevoked => formatter.write_str("Credential has already been revoked"),
            Self::RepositoryFailure => {
                formatter.write_str("Credential repository operation failed")
            }
        }
    }
}

impl Error for EnableCredentialError {}
