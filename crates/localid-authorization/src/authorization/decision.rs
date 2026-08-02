/// Reason why authorization was denied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationDeniedReason {
    /// Requested permission was not granted.
    MissingPermission,

    /// Resource ownership validation failed.
    ResourceOwnershipFailed,

    /// Authorization policy rejected the request.
    PolicyRejected,
}

/// Result of an authorization evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationDecision {
    /// The action is permitted.
    Allowed,

    /// The action is denied.
    Denied {
        /// Reason why authorization failed.
        reason: AuthorizationDeniedReason,
    },
}

impl AuthorizationDecision {
    /// Returns true when authorization is granted.
    #[must_use]
    pub const fn is_allowed(&self) -> bool {
        matches!(self, Self::Allowed)
    }

    /// Returns true when authorization is denied.
    #[must_use]
    pub const fn is_denied(&self) -> bool {
        matches!(self, Self::Denied { .. })
    }

    /// Returns denial reason when authorization is denied.
    #[must_use]
    pub const fn denial_reason(&self) -> Option<&AuthorizationDeniedReason> {
        match self {
            Self::Denied { reason } => Some(reason),
            Self::Allowed => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AuthorizationDecision, AuthorizationDeniedReason};

    #[test]
    fn represents_allowed_decision() {
        let decision = AuthorizationDecision::Allowed;

        assert!(decision.is_allowed());
        assert!(!decision.is_denied());
        assert!(decision.denial_reason().is_none());
    }

    #[test]
    fn represents_denied_decision() {
        let decision = AuthorizationDecision::Denied {
            reason: AuthorizationDeniedReason::MissingPermission,
        };

        assert!(decision.is_denied());
        assert!(!decision.is_allowed());

        assert_eq!(
            decision.denial_reason(),
            Some(&AuthorizationDeniedReason::MissingPermission)
        );
    }
}
