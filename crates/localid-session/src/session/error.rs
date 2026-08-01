use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use super::SessionLifecycleState;

/// Domain errors produced by Session operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionError {
    /// The requested lifecycle transition is not permitted.
    InvalidLifecycleTransition {
        /// Lifecycle state before the requested transition.
        from: SessionLifecycleState,

        /// Requested destination lifecycle state.
        to: SessionLifecycleState,
    },

    /// The expiration time does not occur after the creation time.
    InvalidExpirationTime,
}

impl Display for SessionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLifecycleTransition { from, to } => {
                write!(
                    formatter,
                    "invalid Session lifecycle transition from {from:?} to {to:?}"
                )
            }
            Self::InvalidExpirationTime => {
                formatter.write_str("Session expiration time must occur after its creation time")
            }
        }
    }
}

impl Error for SessionError {}

#[cfg(test)]
mod tests {
    use super::SessionError;
    use crate::SessionLifecycleState;

    #[test]
    fn describes_invalid_lifecycle_transition() {
        let error = SessionError::InvalidLifecycleTransition {
            from: SessionLifecycleState::Revoked,
            to: SessionLifecycleState::Active,
        };

        assert_eq!(
            error.to_string(),
            "invalid Session lifecycle transition from Revoked to Active"
        );
    }

    #[test]
    fn describes_invalid_expiration_time() {
        let error = SessionError::InvalidExpirationTime;

        assert_eq!(
            error.to_string(),
            "Session expiration time must occur after its creation time"
        );
    }

    #[test]
    fn session_errors_are_comparable() {
        assert_eq!(
            SessionError::InvalidExpirationTime,
            SessionError::InvalidExpirationTime
        );

        assert_eq!(
            SessionError::InvalidLifecycleTransition {
                from: SessionLifecycleState::Revoked,
                to: SessionLifecycleState::Active,
            },
            SessionError::InvalidLifecycleTransition {
                from: SessionLifecycleState::Revoked,
                to: SessionLifecycleState::Active,
            }
        );
    }
}
