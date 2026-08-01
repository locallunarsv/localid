use crate::{IdentityId, LifecycleState};

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
}

#[cfg(test)]
mod tests {
    use super::Identity;
    use crate::{IdentityId, LifecycleState};

    #[test]
    fn creates_active_identity() {
        let id = IdentityId::new();

        let identity = Identity::new(id);

        assert_eq!(identity.id(), id);
        assert_eq!(identity.lifecycle_state(), LifecycleState::Active);
    }
}
