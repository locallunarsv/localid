#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LifecycleState {
    #[default]
    Active,
    Disabled,
    Deleted,
}

impl LifecycleState {
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    #[must_use]
    pub const fn is_disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }

    #[must_use]
    pub const fn is_deleted(self) -> bool {
        matches!(self, Self::Deleted)
    }

    #[must_use]
    pub const fn can_authenticate(self) -> bool {
        matches!(self, Self::Active)
    }
}

#[cfg(test)]
mod tests {
    use super::LifecycleState;

    #[test]
    fn active_state_can_authenticate() {
        assert!(LifecycleState::Active.can_authenticate());
    }

    #[test]
    fn disabled_state_cannot_authenticate() {
        assert!(!LifecycleState::Disabled.can_authenticate());
    }

    #[test]
    fn deleted_state_cannot_authenticate() {
        assert!(!LifecycleState::Deleted.can_authenticate());
    }

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
