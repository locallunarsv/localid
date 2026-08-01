use super::{IdentityError, IdentityId, LifecycleState};

/// Canonical and stable representation of a digital subject.
///
/// An Identity owns its identifier and lifecycle state. Credentials, Sessions,
/// profile information, and authentication policies are separate concerns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    id: IdentityId,
    lifecycle_state: LifecycleState,
}

impl Identity {
    /// Creates a new active Identity with the provided identifier.
    #[must_use]
    pub const fn new(id: IdentityId) -> Self {
        Self {
            id,
            lifecycle_state: LifecycleState::Active,
        }
    }

    /// Returns the stable identifier of this Identity.
    #[must_use]
    pub const fn id(&self) -> IdentityId {
        self.id
    }

    /// Returns the current lifecycle state of this Identity.
    #[must_use]
    pub const fn lifecycle_state(&self) -> LifecycleState {
        self.lifecycle_state
    }

    /// Returns `true` when this Identity is active.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.lifecycle_state.is_active()
    }

    /// Returns `true` when this Identity is disabled.
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        self.lifecycle_state.is_disabled()
    }

    /// Returns `true` when this Identity is deleted.
    #[must_use]
    pub const fn is_deleted(&self) -> bool {
        self.lifecycle_state.is_deleted()
    }

    /// Administratively disables this Identity.
    ///
    /// Disabling an already disabled Identity is idempotent.
    ///
    /// # Errors
    ///
    /// Returns [`IdentityError::InvalidLifecycleTransition`] when this Identity
    /// has already been deleted.
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

    /// Enables a previously disabled Identity.
    ///
    /// Enabling an already active Identity is idempotent.
    ///
    /// # Errors
    ///
    /// Returns [`IdentityError::InvalidLifecycleTransition`] when this Identity
    /// has already been deleted.
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

    /// Permanently removes this Identity from operational use.
    ///
    /// Deleting an already deleted Identity is idempotent. A deleted Identity
    /// remains available as a stable historical reference.
    pub const fn delete(&mut self) {
        self.lifecycle_state = LifecycleState::Deleted;
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
