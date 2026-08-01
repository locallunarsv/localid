/// Lifecycle state of a [`Session`](crate::Session).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionLifecycleState {
    /// Session is operational until revoked or expired.
    Active,

    /// Session has been explicitly revoked and cannot become active again.
    Revoked,
}

impl SessionLifecycleState {
    /// Initial lifecycle state assigned to a newly created Session.
    pub const INITIAL: Self = Self::Active;

    /// Returns `true` when the Session is active.
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns `true` when the Session is revoked.
    #[must_use]
    pub const fn is_revoked(self) -> bool {
        matches!(self, Self::Revoked)
    }
}

#[cfg(test)]
mod tests {
    use super::SessionLifecycleState;

    #[test]
    fn initial_state_is_active() {
        assert_eq!(
            SessionLifecycleState::INITIAL,
            SessionLifecycleState::Active
        );
    }

    #[test]
    fn lifecycle_state_predicates_are_correct() {
        assert!(SessionLifecycleState::Active.is_active());
        assert!(SessionLifecycleState::Revoked.is_revoked());

        assert!(!SessionLifecycleState::Active.is_revoked());
        assert!(!SessionLifecycleState::Revoked.is_active());
    }
}
