//! Runtime environment configuration.

use std::{fmt, str::FromStr};

use serde::Deserialize;

/// Runtime environment mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    /// Development mode with demo data.
    Development,

    /// Production mode without demo seed.
    Production,
}

impl Environment {
    /// Returns whether demo seed should be created.
    #[must_use]
    pub const fn should_seed(self) -> bool {
        matches!(self, Self::Development)
    }
}

/// Error returned when parsing an invalid runtime environment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseEnvironmentError {
    value: String,
}

impl fmt::Display for ParseEnvironmentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid runtime environment: {}", self.value)
    }
}

impl std::error::Error for ParseEnvironmentError {}

impl FromStr for Environment {
    type Err = ParseEnvironmentError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            _ => Err(ParseEnvironmentError {
                value: value.to_owned(),
            }),
        }
    }
}
