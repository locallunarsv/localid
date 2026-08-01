use chrono::{TimeDelta, TimeZone, Utc};
use localid_session::SessionId;
use localid_token::{Token, TokenError, TokenId, TokenLifecycleState};

fn creation_time() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 8, 2, 0, 0, 0)
        .single()
        .expect("test timestamp should be valid")
}

#[test]
fn creates_active_token() {
    let id = TokenId::new();
    let session_id = SessionId::new();
    let created_at = creation_time();
    let expires_at = created_at + TimeDelta::hours(1);

    let token = Token::new(
        id,
        session_id,
        "hashed-secret".to_owned(),
        created_at,
        expires_at,
    )
    .expect("expiration after creation should be valid");

    assert_eq!(token.id(), id);
    assert_eq!(token.session_id(), session_id);
    assert_eq!(token.secret_hash(), "hashed-secret");
    assert_eq!(token.lifecycle_state(), TokenLifecycleState::Active);
    assert!(token.is_active());
    assert!(!token.is_revoked());
}

#[test]
fn rejects_expiration_equal_to_creation() {
    let created_at = creation_time();

    let result = Token::new(
        TokenId::new(),
        SessionId::new(),
        "hashed-secret".to_owned(),
        created_at,
        created_at,
    );

    assert_eq!(result, Err(TokenError::InvalidExpirationTime));
}

#[test]
fn rejects_expiration_before_creation() {
    let created_at = creation_time();

    let result = Token::new(
        TokenId::new(),
        SessionId::new(),
        "hashed-secret".to_owned(),
        created_at,
        created_at - TimeDelta::seconds(1),
    );

    assert_eq!(result, Err(TokenError::InvalidExpirationTime));
}

#[test]
fn token_is_valid_before_expiration() {
    let created_at = creation_time();
    let expires_at = created_at + TimeDelta::hours(1);

    let token = Token::new(
        TokenId::new(),
        SessionId::new(),
        "hashed-secret".to_owned(),
        created_at,
        expires_at,
    )
    .expect("expiration after creation should be valid");

    assert!(token.is_valid_at(created_at));
    assert!(token.is_valid_at(expires_at - TimeDelta::seconds(1)));
}

#[test]
fn token_expires_at_expiration_time() {
    let created_at = creation_time();
    let expires_at = created_at + TimeDelta::hours(1);

    let token = Token::new(
        TokenId::new(),
        SessionId::new(),
        "hashed-secret".to_owned(),
        created_at,
        expires_at,
    )
    .expect("expiration after creation should be valid");

    assert!(token.is_expired_at(expires_at));
    assert!(!token.is_valid_at(expires_at));
}

#[test]
fn revokes_active_token() {
    let created_at = creation_time();
    let expires_at = created_at + TimeDelta::hours(1);

    let mut token = Token::new(
        TokenId::new(),
        SessionId::new(),
        "hashed-secret".to_owned(),
        created_at,
        expires_at,
    )
    .expect("expiration after creation should be valid");

    token.revoke();

    assert!(token.is_revoked());
    assert!(!token.is_valid_at(created_at));
}
