/// Lifecycle state of a Token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenLifecycleState {
    /// Token can be used.
    Active,

    /// Token has been revoked.
    Revoked,
}

impl TokenLifecycleState {
    /// Returns whether the Token is active.
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns whether the Token is revoked.
    #[must_use]
    pub const fn is_revoked(self) -> bool {
        matches!(self, Self::Revoked)
    }
}
