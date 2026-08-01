use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// Errors produced by password domain operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordError {
    /// The supplied password secret is empty.
    EmptySecret,
}

impl Display for PasswordError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySecret => formatter.write_str("password secret must not be empty"),
        }
    }
}

impl Error for PasswordError {}
