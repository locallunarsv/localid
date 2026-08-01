use localid_identity::{Identity, IdentityError, IdentityId, LifecycleState};

#[test]
fn creates_an_active_identity() {
    let id = IdentityId::new();

    let identity = Identity::new(id);

    assert_eq!(identity.id(), id);
    assert_eq!(identity.lifecycle_state(), LifecycleState::Active);
    assert!(identity.is_active());
}

#[test]
fn disables_and_enables_an_identity() {
    let mut identity = Identity::new(IdentityId::new());

    identity
        .disable()
        .expect("active Identity should be disableable");

    assert!(identity.is_disabled());

    identity
        .enable()
        .expect("disabled Identity should be enableable");

    assert!(identity.is_active());
}

#[test]
fn deletion_is_terminal() {
    let mut identity = Identity::new(IdentityId::new());

    identity.delete();

    assert!(identity.is_deleted());

    assert_eq!(
        identity.enable(),
        Err(IdentityError::InvalidLifecycleTransition {
            from: LifecycleState::Deleted,
            to: LifecycleState::Active,
        })
    );

    assert_eq!(
        identity.disable(),
        Err(IdentityError::InvalidLifecycleTransition {
            from: LifecycleState::Deleted,
            to: LifecycleState::Disabled,
        })
    );

    assert!(identity.is_deleted());
}

#[test]
fn lifecycle_operations_are_idempotent() {
    let mut identity = Identity::new(IdentityId::new());

    identity
        .disable()
        .expect("active Identity should be disableable");
    identity
        .disable()
        .expect("disabling a disabled Identity should succeed");

    assert!(identity.is_disabled());

    identity
        .enable()
        .expect("disabled Identity should be enableable");
    identity
        .enable()
        .expect("enabling an active Identity should succeed");

    assert!(identity.is_active());

    identity.delete();
    identity.delete();

    assert!(identity.is_deleted());
}
