use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// Errors produced while listing Credentials.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListCredentialsError {
    /// Credential repository operation failed.
    RepositoryFailure,
}

impl Display for ListCredentialsError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RepositoryFailure => {
                formatter.write_str("Credential repository operation failed")
            }
        }
    }
}

impl Error for ListCredentialsError {}
