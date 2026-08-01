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
        let mut identity = Identity {
            id: IdentityId::new(),
            lifecycle_state: LifecycleState::Deleted,
        };

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
}
