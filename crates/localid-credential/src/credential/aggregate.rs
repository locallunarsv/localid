use localid_identity::IdentityId;

use super::{CredentialError, CredentialId, CredentialKind, CredentialLifecycleState};

/// Credential owned by exactly one LocalID Identity.
///
/// A Credential represents one authentication mechanism associated with an
/// Identity. Authentication verification itself remains outside this domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Credential {
    id: CredentialId,
    identity_id: IdentityId,
    kind: CredentialKind,
    lifecycle_state: CredentialLifecycleState,
}

impl Credential {
    /// Creates a new active Credential.
    #[must_use]
    pub const fn new(id: CredentialId, identity_id: IdentityId, kind: CredentialKind) -> Self {
        Self {
            id,
            identity_id,
            kind,
            lifecycle_state: CredentialLifecycleState::INITIAL,
        }
    }

    /// Returns this Credential's stable identifier.
    #[must_use]
    pub const fn id(&self) -> CredentialId {
        self.id
    }

    /// Returns the identifier of the owning Identity.
    #[must_use]
    pub const fn identity_id(&self) -> IdentityId {
        self.identity_id
    }

    /// Returns the authentication mechanism represented by this Credential.
    #[must_use]
    pub const fn kind(&self) -> CredentialKind {
        self.kind
    }

    /// Returns the current Credential lifecycle state.
    #[must_use]
    pub const fn lifecycle_state(&self) -> CredentialLifecycleState {
        self.lifecycle_state
    }

    /// Returns `true` when this Credential is active.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.lifecycle_state.is_active()
    }

    /// Returns `true` when this Credential is disabled.
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        self.lifecycle_state.is_disabled()
    }

    /// Returns `true` when this Credential is revoked.
    #[must_use]
    pub const fn is_revoked(&self) -> bool {
        self.lifecycle_state.is_revoked()
    }

    /// Temporarily disables this Credential.
    ///
    /// Disabling an already disabled Credential is idempotent.
    ///
    /// # Errors
    ///
    /// Returns [`CredentialError::InvalidLifecycleTransition`] when this
    /// Credential has already been revoked.
    pub const fn disable(&mut self) -> Result<(), CredentialError> {
        match self.lifecycle_state {
            CredentialLifecycleState::Active => {
                self.lifecycle_state = CredentialLifecycleState::Disabled;
                Ok(())
            }
            CredentialLifecycleState::Disabled => Ok(()),
            CredentialLifecycleState::Revoked => Err(CredentialError::InvalidLifecycleTransition {
                from: CredentialLifecycleState::Revoked,
                to: CredentialLifecycleState::Disabled,
            }),
        }
    }
    /// Enables a previously disabled Credential.
    ///
    /// Enabling an already active Credential is idempotent.
    ///
    /// # Errors
    ///
    /// Returns [`CredentialError::InvalidLifecycleTransition`] when this
    /// Credential has already been revoked.
    pub const fn enable(&mut self) -> Result<(), CredentialError> {
        match self.lifecycle_state {
            CredentialLifecycleState::Active => Ok(()),
            CredentialLifecycleState::Disabled => {
                self.lifecycle_state = CredentialLifecycleState::Active;
                Ok(())
            }
            CredentialLifecycleState::Revoked => Err(CredentialError::InvalidLifecycleTransition {
                from: CredentialLifecycleState::Revoked,
                to: CredentialLifecycleState::Active,
            }),
        }
    }
    /// Permanently revokes this Credential.
    ///
    /// Revoking an already revoked Credential is idempotent.
    pub const fn revoke(&mut self) {
        self.lifecycle_state = CredentialLifecycleState::Revoked;
    }
}

#[cfg(test)]
mod tests {
    use localid_identity::IdentityId;

    use super::Credential;
    use crate::{CredentialError, CredentialId, CredentialKind, CredentialLifecycleState};

    #[test]
    fn creates_active_credential() {
        let id = CredentialId::new();
        let identity_id = IdentityId::new();

        let credential = Credential::new(id, identity_id, CredentialKind::Password);

        assert_eq!(credential.id(), id);
        assert_eq!(credential.identity_id(), identity_id);
        assert_eq!(credential.kind(), CredentialKind::Password);
        assert_eq!(
            credential.lifecycle_state(),
            CredentialLifecycleState::Active
        );
        assert!(credential.is_active());
        assert!(!credential.is_disabled());
        assert!(!credential.is_revoked());
    }
    #[test]
    fn disables_active_credential() {
        let mut credential = Credential::new(
            CredentialId::new(),
            IdentityId::new(),
            CredentialKind::Password,
        );

        credential
            .disable()
            .expect("active Credential should be disableable");

        assert!(credential.is_disabled());
    }

    #[test]
    fn disabling_disabled_credential_is_idempotent() {
        let mut credential = Credential::new(
            CredentialId::new(),
            IdentityId::new(),
            CredentialKind::Password,
        );

        credential
            .disable()
            .expect("active Credential should be disableable");

        credential
            .disable()
            .expect("disabling a disabled Credential should succeed");

        assert!(credential.is_disabled());
    }
    #[test]
    fn enables_disabled_credential() {
        let mut credential = Credential::new(
            CredentialId::new(),
            IdentityId::new(),
            CredentialKind::Password,
        );

        credential
            .disable()
            .expect("active Credential should be disableable");

        credential
            .enable()
            .expect("disabled Credential should be enableable");

        assert!(credential.is_active());
    }

    #[test]
    fn enabling_active_credential_is_idempotent() {
        let mut credential = Credential::new(
            CredentialId::new(),
            IdentityId::new(),
            CredentialKind::Password,
        );

        credential
            .enable()
            .expect("enabling an active Credential should succeed");

        assert!(credential.is_active());
    }
    #[test]
    fn revokes_active_credential() {
        let mut credential = Credential::new(
            CredentialId::new(),
            IdentityId::new(),
            CredentialKind::Password,
        );

        credential.revoke();

        assert!(credential.is_revoked());
    }

    #[test]
    fn revokes_disabled_credential() {
        let mut credential = Credential::new(
            CredentialId::new(),
            IdentityId::new(),
            CredentialKind::Password,
        );

        credential
            .disable()
            .expect("active Credential should be disableable");

        credential.revoke();

        assert!(credential.is_revoked());
    }

    #[test]
    fn revoking_revoked_credential_is_idempotent() {
        let mut credential = Credential::new(
            CredentialId::new(),
            IdentityId::new(),
            CredentialKind::Password,
        );

        credential.revoke();
        credential.revoke();

        assert!(credential.is_revoked());
    }

    #[test]
    fn cannot_disable_revoked_credential() {
        let mut credential = Credential::new(
            CredentialId::new(),
            IdentityId::new(),
            CredentialKind::Password,
        );

        credential.revoke();

        let result = credential.disable();

        assert_eq!(
            result,
            Err(CredentialError::InvalidLifecycleTransition {
                from: CredentialLifecycleState::Revoked,
                to: CredentialLifecycleState::Disabled,
            })
        );
        assert!(credential.is_revoked());
    }

    #[test]
    fn cannot_enable_revoked_credential() {
        let mut credential = Credential::new(
            CredentialId::new(),
            IdentityId::new(),
            CredentialKind::Password,
        );

        credential.revoke();

        let result = credential.enable();

        assert_eq!(
            result,
            Err(CredentialError::InvalidLifecycleTransition {
                from: CredentialLifecycleState::Revoked,
                to: CredentialLifecycleState::Active,
            })
        );
        assert!(credential.is_revoked());
    }
}
