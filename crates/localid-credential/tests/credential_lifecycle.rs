use localid_credential::{
    Credential, CredentialError, CredentialId, CredentialKind, CredentialLifecycleState,
};
use localid_identity::IdentityId;

#[test]
fn creates_an_active_credential() {
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
}

#[test]
fn disables_and_enables_a_credential() {
    let mut credential = Credential::new(
        CredentialId::new(),
        IdentityId::new(),
        CredentialKind::Passkey,
    );

    credential
        .disable()
        .expect("active Credential should be disableable");

    assert!(credential.is_disabled());

    credential
        .enable()
        .expect("disabled Credential should be enableable");

    assert!(credential.is_active());
}

#[test]
fn revocation_is_terminal() {
    let mut credential = Credential::new(
        CredentialId::new(),
        IdentityId::new(),
        CredentialKind::ApiKey,
    );

    credential.revoke();

    assert!(credential.is_revoked());

    assert_eq!(
        credential.enable(),
        Err(CredentialError::InvalidLifecycleTransition {
            from: CredentialLifecycleState::Revoked,
            to: CredentialLifecycleState::Active,
        })
    );

    assert_eq!(
        credential.disable(),
        Err(CredentialError::InvalidLifecycleTransition {
            from: CredentialLifecycleState::Revoked,
            to: CredentialLifecycleState::Disabled,
        })
    );

    assert!(credential.is_revoked());
}

#[test]
fn lifecycle_operations_are_idempotent() {
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

    credential
        .enable()
        .expect("disabled Credential should be enableable");
    credential
        .enable()
        .expect("enabling an active Credential should succeed");

    assert!(credential.is_active());

    credential.revoke();
    credential.revoke();

    assert!(credential.is_revoked());
}
