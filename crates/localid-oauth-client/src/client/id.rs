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
