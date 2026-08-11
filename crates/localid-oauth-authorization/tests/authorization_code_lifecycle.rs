use chrono::{TimeDelta, TimeZone, Utc};

use localid_identity::IdentityId;
use localid_oauth_authorization::{
    AuthorizationCode, AuthorizationCodeError, AuthorizationCodeId, AuthorizationCodeLifecycleState,
};
use localid_oauth_client::OAuthClientId;

fn create_code() -> AuthorizationCode {
    let created_at = Utc
        .with_ymd_and_hms(2026, 8, 2, 0, 0, 0)
        .single()
        .expect("timestamp should be valid");

    AuthorizationCode::new(
        AuthorizationCodeId::new(),
        OAuthClientId::new(),
        IdentityId::new(),
        "code-hash",
        "http://localhost:3000/callback",
        vec!["openid".to_string()],
        created_at,
        created_at + TimeDelta::minutes(10),
    )
    .expect("authorization code should be valid")
}

#[test]
fn creates_active_authorization_code() {
    let code = create_code();

    assert_eq!(code.state(), AuthorizationCodeLifecycleState::Active);

    assert!(code.state().is_active());
}

#[test]
fn rejects_invalid_expiration() {
    let created_at = Utc
        .with_ymd_and_hms(2026, 8, 2, 0, 0, 0)
        .single()
        .expect("timestamp should be valid");

    let result = AuthorizationCode::new(
        AuthorizationCodeId::new(),
        OAuthClientId::new(),
        IdentityId::new(),
        "code-hash",
        "http://localhost:3000/callback",
        vec!["openid".to_string()],
        created_at,
        created_at,
    );

    assert_eq!(result, Err(AuthorizationCodeError::InvalidExpirationTime));
}

#[test]
fn consumes_authorization_code() {
    let mut code = create_code();

    code.consume().expect("code should be consumed");

    assert_eq!(code.state(), AuthorizationCodeLifecycleState::Consumed);

    assert!(code.state().is_consumed());
}

#[test]
fn cannot_consume_twice() {
    let mut code = create_code();

    code.consume().expect("first consume should succeed");

    let result = code.consume();

    assert_eq!(result, Err(AuthorizationCodeError::AlreadyConsumed));
}

#[test]
fn expires_after_expiration_time() {
    let code = create_code();

    let after_expiration = code.expires_at() + TimeDelta::seconds(1);

    assert!(code.is_expired_at(after_expiration));

    assert!(!code.is_active_at(after_expiration));
}

#[test]
fn stores_requested_scope() {
    let code = create_code();

    assert_eq!(code.scope(), &["openid".to_string()]);
}
