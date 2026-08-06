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

    /// Returns whether client is disabled.
    #[must_use]
    pub const fn is_disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }

    /// Returns whether client is deleted.
    #[must_use]
    pub const fn is_deleted(self) -> bool {
        matches!(self, Self::Deleted)
    }
}
