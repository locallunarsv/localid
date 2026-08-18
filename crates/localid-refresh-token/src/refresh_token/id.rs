use uuid::Uuid;

/// Stable identifier for a Refresh Token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefreshTokenId(Uuid);

impl RefreshTokenId {
    /// Restores RefreshToken identifier from UUID.
    #[must_use]
    pub const fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    /// Returns underlying UUID.
    #[must_use]
    pub const fn as_uuid(self) -> Uuid {
        self.0
    }
}

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
