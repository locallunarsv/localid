use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use super::CredentialLifecycleState;

/// Domain errors produced by Credential lifecycle operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialError {
    /// The requested lifecycle transition is not permitted.
    InvalidLifecycleTransition {
        /// Lifecycle state before the requested transition.
        from: CredentialLifecycleState,

        /// Requested destination lifecycle state.
        to: CredentialLifecycleState,
    },
}

impl Display for CredentialError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLifecycleTransition { from, to } => {
                write!(
                    formatter,
                    "invalid Credential lifecycle transition from {from:?} to {to:?}"
                )
            }
        }
    }
}

impl Error for CredentialError {}

#[cfg(test)]
mod tests {
    use super::CredentialError;
    use crate::CredentialLifecycleState;

    #[test]
    fn describes_invalid_lifecycle_transition() {
        let error = CredentialError::InvalidLifecycleTransition {
            from: CredentialLifecycleState::Revoked,
            to: CredentialLifecycleState::Active,
        };

        assert_eq!(
            error.to_string(),
            "invalid Credential lifecycle transition from Revoked to Active"
        );
    }

    #[test]
    fn invalid_transition_errors_are_comparable() {
        let first = CredentialError::InvalidLifecycleTransition {
            from: CredentialLifecycleState::Revoked,
            to: CredentialLifecycleState::Active,
        };

        let second = CredentialError::InvalidLifecycleTransition {
            from: CredentialLifecycleState::Revoked,
            to: CredentialLifecycleState::Active,
        };

        assert_eq!(first, second);
    }
}
