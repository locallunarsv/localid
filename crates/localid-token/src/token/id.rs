use uuid::Uuid;

/// Stable identifier for a Token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenId(Uuid);

impl TokenId {
    /// Creates a new Token identifier.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for TokenId {
    fn default() -> Self {
        Self::new()
    }
}
