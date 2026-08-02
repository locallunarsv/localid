use chrono::{Duration, Utc};

use localid_authentication::{DefaultRefreshTokenService, RefreshTokenService};
use localid_identity::IdentityId;
use localid_refresh_token_random::{RandomRefreshTokenIssuer, RefreshTokenIssuer};
use localid_repository::{RefreshTokenRepository, SessionRepository};
use localid_session::{Session, SessionId};
use localid_storage_memory::MemoryStorage;
use localid_token_random::RandomTokenIssuer;

#[test]
fn refreshes_valid_refresh_token() {
    let mut storage = MemoryStorage::new();

    let now = Utc::now();

    let session = Session::new(
        SessionId::new(),
        IdentityId::new(),
        now,
        now + Duration::hours(1),
    )
    .expect("session should be valid");

    SessionRepository::save(&mut storage, session.clone()).expect("session should save");

    let issued_refresh_token = RandomRefreshTokenIssuer::new()
        .issue(session.id(), now + Duration::days(30))
        .expect("refresh token should issue");

    let secret = issued_refresh_token.secret().to_owned();
    let old_refresh_token_id = issued_refresh_token.token().id();
    let session_id = session.id();

    RefreshTokenRepository::save(&mut storage, issued_refresh_token.token().clone())
        .expect("refresh token should save");

    let mut service = DefaultRefreshTokenService::new(
        storage.clone(),
        storage.clone(),
        storage.clone(),
        RandomRefreshTokenIssuer::new(),
        RandomTokenIssuer::new(),
    );

    let result = service.refresh(&secret).expect("refresh should succeed");

    assert_ne!(result.refresh_token().secret(), secret);

    let old_token = RefreshTokenRepository::find_by_id(&storage, old_refresh_token_id)
        .expect("old token lookup should work")
        .expect("old token should exist");

    assert!(!old_token.is_active());

    assert_eq!(result.refresh_token().token().session_id(), session.id());

    assert_eq!(result.access_token().token().session_id(), session.id());

    let stored_token = localid_repository::TokenRepository::find_by_id(
        &storage,
        result.access_token().token().id(),
    )
    .expect("token lookup should work")
    .expect("token should exist");

    assert_eq!(stored_token.session_id(), session_id);
}
