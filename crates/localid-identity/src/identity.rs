use crate::{IdentityError, IdentityId, LifecycleState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    id: IdentityId,
    lifecycle_state: LifecycleState,
}

impl Identity {
    #[must_use]
    pub const fn new(id: IdentityId) -> Self {
        Self {
            id,
            lifecycle_state: LifecycleState::Active,
        }
    }

    #[must_use]
    pub const fn id(&self) -> IdentityId {
        self.id
    }

    #[must_use]
    pub const fn lifecycle_state(&self) -> LifecycleState {
        self.lifecycle_state
    }

    pub const fn disable(&mut self) -> Result<(), IdentityError> {
        match self.lifecycle_state {
            LifecycleState::Active => {
                self.lifecycle_state = LifecycleState::Disabled;
                Ok(())
            }
            LifecycleState::Disabled => Ok(()),
            LifecycleState::Deleted => Err(IdentityError::InvalidLifecycleTransition {
                from: LifecycleState::Deleted,
                to: LifecycleState::Disabled,
            }),
        }
    }

    pub const fn enable(&mut self) -> Result<(), IdentityError> {
        match self.lifecycle_state {
            LifecycleState::Active => Ok(()),
            LifecycleState::Disabled => {
                self.lifecycle_state = LifecycleState::Active;
                Ok(())
            }
            LifecycleState::Deleted => Err(IdentityError::InvalidLifecycleTransition {
                from: LifecycleState::Deleted,
                to: LifecycleState::Active,
            }),
        }
    }

    pub const fn delete(&mut self) {
        self.lifecycle_state = LifecycleState::Deleted;
    }

    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.lifecycle_state.is_active()
    }

    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        self.lifecycle_state.is_disabled()
    }

    #[must_use]
    pub const fn is_deleted(&self) -> bool {
        self.lifecycle_state.is_deleted()
    }
}

#[cfg(test)]
mod tests {
    use super::Identity;
    use crate::{IdentityError, IdentityId, LifecycleState};

    #[test]
    fn creates_active_identity() {
        let id = IdentityId::new();

        let identity = Identity::new(id);

        assert_eq!(identity.id(), id);
        assert_eq!(identity.lifecycle_state(), LifecycleState::Active);
    }

    #[test]
    fn disables_active_identity() {
        let mut identity = Identity::new(IdentityId::new());

        identity
            .disable()
            .expect("active Identity should be disableable");

        assert_eq!(identity.lifecycle_state(), LifecycleState::Disabled);
    }

    #[test]
    fn disabling_disabled_identity_is_idempotent() {
        let mut identity = Identity::new(IdentityId::new());

        identity
            .disable()
            .expect("active Identity should be disableable");

        identity
            .disable()
            .expect("disabling an already disabled Identity should succeed");

        assert_eq!(identity.lifecycle_state(), LifecycleState::Disabled);
    }

    #[test]
    fn cannot_disable_deleted_identity() {
        let mut identity = Identity::new(IdentityId::new());
        identity.delete();

        let result = identity.disable();

        assert_eq!(
            result,
            Err(IdentityError::InvalidLifecycleTransition {
                from: LifecycleState::Deleted,
                to: LifecycleState::Disabled,
            })
        );
        assert_eq!(identity.lifecycle_state(), LifecycleState::Deleted);
    }

    #[test]
    fn enables_disabled_identity() {
        let mut identity = Identity::new(IdentityId::new());

        identity
            .disable()
            .expect("active Identity should be disableable");

        identity
            .enable()
            .expect("disabled Identity should be enableable");

        assert_eq!(identity.lifecycle_state(), LifecycleState::Active);
    }

    #[test]
    fn enabling_active_identity_is_idempotent() {
        let mut identity = Identity::new(IdentityId::new());

        identity
            .enable()
            .expect("enabling an active Identity should succeed");

        assert_eq!(identity.lifecycle_state(), LifecycleState::Active);
    }

    #[test]
    fn cannot_enable_deleted_identity() {
        let mut identity = Identity::new(IdentityId::new());
        identity.delete();

        let result = identity.enable();

        assert_eq!(
            result,
            Err(IdentityError::InvalidLifecycleTransition {
                from: LifecycleState::Deleted,
                to: LifecycleState::Active,
            })
        );
        assert_eq!(identity.lifecycle_state(), LifecycleState::Deleted);
    }

    #[test]
    fn deletes_active_identity() {
        let mut identity = Identity::new(IdentityId::new());

        identity.delete();

        assert_eq!(identity.lifecycle_state(), LifecycleState::Deleted);
    }

    #[test]
    fn deletes_disabled_identity() {
        let mut identity = Identity::new(IdentityId::new());

        identity
            .disable()
            .expect("active Identity should be disableable");

        identity.delete();

        assert_eq!(identity.lifecycle_state(), LifecycleState::Deleted);
    }

    #[test]
    fn deleting_deleted_identity_is_idempotent() {
        let mut identity = Identity::new(IdentityId::new());

        identity.delete();
        identity.delete();

        assert_eq!(identity.lifecycle_state(), LifecycleState::Deleted);
    }

    #[test]
    fn reports_current_lifecycle_state() {
        let mut identity = Identity::new(IdentityId::new());

        assert!(identity.is_active());
        assert!(!identity.is_disabled());
        assert!(!identity.is_deleted());

        identity
            .disable()
            .expect("active Identity should be disableable");

        assert!(!identity.is_active());
        assert!(identity.is_disabled());
        assert!(!identity.is_deleted());

        identity.delete();

        assert!(!identity.is_active());
        assert!(!identity.is_disabled());
        assert!(identity.is_deleted());
    }
}
