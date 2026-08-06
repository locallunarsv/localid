use std::{fmt, str::FromStr};

use uuid::Uuid;

/// Unique identifier for a client application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientId(Uuid);

impl ClientId {
    /// Creates a new client identifier.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for ClientId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for ClientId {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(value)?))
    }
}

impl fmt::Display for ClientId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}
