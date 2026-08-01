use chrono::{TimeDelta, TimeZone, Utc};
use localid_identity::IdentityId;
use localid_session::{Session, SessionError, SessionId, SessionLifecycleState};

fn creation_time() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 8, 2, 0, 0, 0)
        .single()
        .expect("test timestamp should be valid")
}

#[test]
fn creates_active_session() {
    let created_at = creation_time();
    let expires_at = created_at + TimeDelta::hours(1);

    let session = Session::new(SessionId::new(), IdentityId::new(), created_at, expires_at)
        .expect("valid session should be created");

    assert_eq!(session.lifecycle_state(), SessionLifecycleState::Active);

    assert!(session.is_active());
    assert!(session.is_valid_at(created_at));
}

#[test]
fn rejects_invalid_expiration() {
    let created_at = creation_time();

    let result = Session::new(SessionId::new(), IdentityId::new(), created_at, created_at);

    assert_eq!(result, Err(SessionError::InvalidExpirationTime));
}

#[test]
fn expiration_is_time_based() {
    let created_at = creation_time();
    let expires_at = created_at + TimeDelta::minutes(30);

    let session = Session::new(SessionId::new(), IdentityId::new(), created_at, expires_at)
        .expect("valid session should be created");

    assert!(session.is_valid_at(created_at));

    let after_expiration = expires_at + TimeDelta::seconds(1);

    assert!(session.is_expired_at(after_expiration));
    assert!(!session.is_valid_at(after_expiration));
}

#[test]
fn revocation_invalidates_session() {
    let created_at = creation_time();
    let expires_at = created_at + TimeDelta::hours(1);

    let mut session = Session::new(SessionId::new(), IdentityId::new(), created_at, expires_at)
        .expect("valid session should be created");

    session.revoke();

    assert!(session.is_revoked());
    assert!(!session.is_valid_at(created_at));
}
