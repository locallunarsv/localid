use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// Errors produced while getting a Credential.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GetCredentialError {
    /// Credential was not found.
    NotFound,

    /// Credential repository operation failed.
    RepositoryFailure,
}

impl Display for GetCredentialError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => formatter.write_str("Credential not found"),
            Self::RepositoryFailure => {
                formatter.write_str("Credential repository operation failed")
            }
        }
    }
}

impl Error for GetCredentialError {}
