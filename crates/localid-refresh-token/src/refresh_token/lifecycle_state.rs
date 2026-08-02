/// Lifecycle state of a Refresh Token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefreshTokenLifecycleState {
    /// Token can be used.
    Active,

    /// Token has been revoked.
    Revoked,
}

impl RefreshTokenLifecycleState {
    /// Returns true when active.
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns true when revoked.
    #[must_use]
    pub const fn is_revoked(self) -> bool {
        matches!(self, Self::Revoked)
    }
}
