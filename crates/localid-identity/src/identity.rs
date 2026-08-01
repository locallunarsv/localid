use crate::{IdentityId, LifecycleState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    id: IdentityId,
    lifecycle_state: LifecycleState,
}
