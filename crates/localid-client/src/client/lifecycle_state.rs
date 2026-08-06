/// Lifecycle state of a client application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientLifecycleState {
    /// Client can authenticate users.
    Active,

    /// Client is temporarily disabled.
    Disabled,

    /// Client has been permanently deleted.
    Deleted,
}

impl ClientLifecycleState {
    /// Returns whether client is active.
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }
}
