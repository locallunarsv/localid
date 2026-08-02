use uuid::Uuid;

/// Stable identifier for a Refresh Token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefreshTokenId(Uuid);

impl RefreshTokenId {
    /// Creates a new identifier.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for RefreshTokenId {
    fn default() -> Self {
        Self::new()
    }
}
