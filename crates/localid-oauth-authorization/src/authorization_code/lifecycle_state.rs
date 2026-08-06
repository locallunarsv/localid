/// Lifecycle state of an authorization code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationCodeLifecycleState {
    /// Code can still be exchanged.
    Active,

    /// Code has already been consumed.
    Consumed,

    /// Code expired.
    Expired,
}

impl AuthorizationCodeLifecycleState {
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    #[must_use]
    pub const fn is_consumed(self) -> bool {
        matches!(self, Self::Consumed)
    }
}
