use uuid::Uuid;

/// Unique identifier for an OAuth client.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OAuthClientId(Uuid);

impl OAuthClientId {
    /// Creates a new OAuth client identifier.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Creates an OAuth client identifier from UUID.
    #[must_use]
    pub const fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    /// Returns underlying UUID value.
    #[must_use]
    pub const fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for OAuthClientId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for OAuthClientId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::str::FromStr for OAuthClientId {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(value)?))
    }
}
