use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// Errors that may occur during Token operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenError {
    /// Token expiration is invalid.
    InvalidExpirationTime,
}

impl Display for TokenError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidExpirationTime => formatter.write_str("invalid token expiration time"),
        }
    }
}

impl Error for TokenError {}
