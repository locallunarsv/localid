use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use super::LifecycleState;

/// Domain errors produced by Identity lifecycle operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentityError {
    /// The requested lifecycle transition is not permitted.
    InvalidLifecycleTransition {
        /// Lifecycle state before the requested transition.
        from: LifecycleState,

        /// Requested destination lifecycle state.
        to: LifecycleState,
    },
}

impl Display for IdentityError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLifecycleTransition { from, to } => {
                write!(
                    formatter,
                    "invalid Identity lifecycle transition from {from:?} to {to:?}"
                )
            }
        }
    }
}

impl Error for IdentityError {}

#[cfg(test)]
mod tests {
    use super::IdentityError;
    use crate::LifecycleState;

    #[test]
    fn describes_invalid_lifecycle_transition() {
        let error = IdentityError::InvalidLifecycleTransition {
            from: LifecycleState::Deleted,
            to: LifecycleState::Active,
        };

        assert_eq!(
            error.to_string(),
            "invalid Identity lifecycle transition from Deleted to Active"
        );
    }

    #[test]
    fn invalid_transition_errors_are_comparable() {
        let first = IdentityError::InvalidLifecycleTransition {
            from: LifecycleState::Deleted,
            to: LifecycleState::Active,
        };

        let second = IdentityError::InvalidLifecycleTransition {
            from: LifecycleState::Deleted,
            to: LifecycleState::Active,
        };

        assert_eq!(first, second);
    }
}
