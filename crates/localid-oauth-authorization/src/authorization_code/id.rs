use uuid::Uuid;

/// Unique identifier for an authorization code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuthorizationCodeId(Uuid);

impl AuthorizationCodeId {
    /// Creates a new authorization code identifier.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for AuthorizationCodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for AuthorizationCodeId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::str::FromStr for AuthorizationCodeId {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(value)?))
    }
}
