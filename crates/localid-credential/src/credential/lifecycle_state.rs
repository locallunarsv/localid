/// Lifecycle state of a [`Credential`](crate::Credential).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CredentialLifecycleState {
    /// Credential is available for verification.
    Active,

    /// Credential is temporarily unavailable.
    Disabled,

    /// Credential is permanently unavailable.
    Revoked,
}

impl CredentialLifecycleState {
    /// Initial lifecycle state assigned to a newly created Credential.
    pub const INITIAL: Self = Self::Active;

    /// Returns `true` when the Credential is active.
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns `true` when the Credential is disabled.
    #[must_use]
    pub const fn is_disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }

    /// Returns `true` when the Credential is revoked.
    #[must_use]
    pub const fn is_revoked(self) -> bool {
        matches!(self, Self::Revoked)
    }
}

#[cfg(test)]
mod tests {
    use super::CredentialLifecycleState;

    #[test]
    fn initial_state_is_active() {
        assert_eq!(
            CredentialLifecycleState::INITIAL,
            CredentialLifecycleState::Active
        );
    }

    #[test]
    fn lifecycle_state_predicates_are_correct() {
        assert!(CredentialLifecycleState::Active.is_active());
        assert!(CredentialLifecycleState::Disabled.is_disabled());
        assert!(CredentialLifecycleState::Revoked.is_revoked());

        assert!(!CredentialLifecycleState::Active.is_disabled());
        assert!(!CredentialLifecycleState::Disabled.is_revoked());
        assert!(!CredentialLifecycleState::Revoked.is_active());
    }
}
