use chrono::{TimeDelta, Utc};
use localid_authentication::{DefaultTokenValidator, TokenValidator};
use localid_client::ClientId;
use localid_repository::{SessionRepository, TokenRepository};
use localid_session::{Session, SessionId};
use localid_storage_memory::MemoryStorage;
use localid_token::{Token, TokenId};
use sha2::{Digest, Sha256};

fn hash_secret(secret: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(secret.as_bytes());

    hex::encode(hasher.finalize())
}

#[test]
fn validates_active_token() {
    let mut storage = MemoryStorage::new();

    let created_at = Utc::now();

    let session = Session::new(
        SessionId::new(),
        localid_identity::IdentityId::new(),
        ClientId::new(),
        created_at,
        created_at + TimeDelta::hours(1),
    )
    .expect("session should be valid");

    let session_id = session.id();

    SessionRepository::save(&mut storage, session.clone()).expect("session should save");

    let secret = "test-secret";

    let token = Token::new(
        TokenId::new(),
        session_id,
        hash_secret(secret),
        created_at,
        created_at + TimeDelta::hours(1),
    )
    .expect("token should be valid");

    TokenRepository::save(&mut storage, token).expect("token should save");

    let validator = DefaultTokenValidator::new(storage.clone(), storage.clone());

    let context = validator.validate(secret).expect("token should validate");

    assert_eq!(context.session_id(), session_id);
}

#[test]
fn rejects_unknown_token() {
    let storage = MemoryStorage::new();

    let validator = DefaultTokenValidator::new(storage.clone(), storage);

    let result = validator.validate("unknown-secret");

    assert_eq!(
        result.unwrap_err(),
        localid_authentication::AuthenticationError::TokenNotFound
    );
}

#[test]
fn rejects_expired_token() {
    let mut storage = MemoryStorage::new();

    let created_at = Utc::now() - TimeDelta::hours(2);

    let session = Session::new(
        SessionId::new(),
        localid_identity::IdentityId::new(),
        ClientId::new(),
        created_at,
        created_at + TimeDelta::hours(1),
    )
    .expect("session should be valid");

    SessionRepository::save(&mut storage, session.clone()).expect("session should save");

    let secret = "expired-secret";

    let token = Token::new(
        TokenId::new(),
        session.id(),
        hash_secret(secret),
        created_at,
        created_at + TimeDelta::hours(1),
    )
    .expect("token should be valid");

    TokenRepository::save(&mut storage, token).expect("token should save");

    let validator = DefaultTokenValidator::new(storage.clone(), storage);

    let result = validator.validate(secret);

    assert_eq!(
        result.unwrap_err(),
        localid_authentication::AuthenticationError::TokenUnavailable
    );
}

#[test]
fn rejects_invalid_session() {
    let mut storage = MemoryStorage::new();

    let created_at = Utc::now();

    let mut session = Session::new(
        SessionId::new(),
        localid_identity::IdentityId::new(),
        ClientId::new(),
        created_at,
        created_at + TimeDelta::hours(1),
    )
    .expect("session should be valid");

    session.revoke();

    let secret = "revoked-session-secret";

    let token = Token::new(
        TokenId::new(),
        session.id(),
        hash_secret(secret),
        created_at,
        created_at + TimeDelta::hours(1),
    )
    .expect("token should be valid");

    SessionRepository::save(&mut storage, session).expect("session should save");

    TokenRepository::save(&mut storage, token).expect("token should save");

    let validator = DefaultTokenValidator::new(storage.clone(), storage);

    let result = validator.validate(secret);

    assert_eq!(
        result.unwrap_err(),
        localid_authentication::AuthenticationError::SessionUnavailable
    );
}
