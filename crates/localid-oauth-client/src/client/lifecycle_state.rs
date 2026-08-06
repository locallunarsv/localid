/// Lifecycle state of an OAuth client.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OAuthClientLifecycleState {
    /// Client can request authorization.
    Active,

    /// Client is temporarily disabled.
    Disabled,

    /// Client is permanently deleted.
    Deleted,
}

impl OAuthClientLifecycleState {
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }
}
