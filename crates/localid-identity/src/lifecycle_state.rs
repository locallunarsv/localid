/// Lifecycle state of an [`Identity`](crate::Identity).
///
/// Lifecycle state represents whether an Identity is operational,
/// administratively unavailable, or permanently removed from operational use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LifecycleState {
    /// The Identity is available for normal operational use.
    #[default]
    Active,

    /// The Identity is administratively unavailable.
    ///
    /// A disabled Identity still exists and may later be enabled again.
    Disabled,

    /// The Identity has been permanently removed from operational use.
    ///
    /// Deleted is a terminal lifecycle state.
    Deleted,
}

impl LifecycleState {
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
    fn default_state_is_active() {
        assert_eq!(LifecycleState::default(), LifecycleState::Active);
    }

    #[test]
    fn state_predicates_are_correct() {
        assert!(LifecycleState::Active.is_active());
        assert!(LifecycleState::Disabled.is_disabled());
        assert!(LifecycleState::Deleted.is_deleted());
    }
}
