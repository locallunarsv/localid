use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// Errors that may occur during Refresh Token operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefreshTokenError {
    /// Expiration time is invalid.
    InvalidExpirationTime,
}

impl Display for RefreshTokenError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidExpirationTime => {
                formatter.write_str("invalid refresh token expiration time")
            }
        }
    }
}

impl Error for RefreshTokenError {}
