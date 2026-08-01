/// Lifecycle state of an [`Identity`](crate::Identity).
///
/// Lifecycle state represents whether an Identity is operational,
/// administratively unavailable, or permanently removed from operational use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LifecycleState {
    /// The Identity is available for normal operational use.
    Active,

    /// The Identity is administratively unavailable.
    Disabled,

    /// The Identity has been permanently removed from operational use.
    Deleted,
}

impl LifecycleState {
    /// Initial lifecycle state assigned to a newly created Identity.
    pub const INITIAL: Self = Self::Active;

    /// Returns `true` when the lifecycle state is [`Active`](Self::Active).
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns `true` when the lifecycle state is [`Disabled`](Self::Disabled).
    #[must_use]
    pub const fn is_disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }

    /// Returns `true` when the lifecycle state is [`Deleted`](Self::Deleted).
    #[must_use]
    pub const fn is_deleted(self) -> bool {
        matches!(self, Self::Deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::LifecycleState;

    #[test]
    fn initial_state_is_active() {
        assert_eq!(LifecycleState::INITIAL, LifecycleState::Active);
    }

    #[test]
    fn state_predicates_are_correct() {
        assert!(LifecycleState::Active.is_active());
        assert!(LifecycleState::Disabled.is_disabled());
        assert!(LifecycleState::Deleted.is_deleted());
    }
}
