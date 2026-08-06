use chrono::{TimeDelta, Utc};

use localid_identity::IdentityId;
use localid_oauth_authorization::{AuthorizationCode, AuthorizationCodeId};
use localid_oauth_authorization_repository_memory::MemoryAuthorizationCodeRepository;
use localid_oauth_client::OAuthClientId;

use localid_oauth_authorization::AuthorizationCodeRepository;

fn authorization_code() -> AuthorizationCode {
    let created_at = Utc::now();

    AuthorizationCode::new(
        AuthorizationCodeId::new(),
        OAuthClientId::new(),
        IdentityId::new(),
        "hash",
        "http://localhost/callback",
        created_at,
        created_at + TimeDelta::minutes(10),
    )
    .expect("authorization code should be valid")
}

#[test]
fn saves_and_finds_authorization_code() {
    let mut repository = MemoryAuthorizationCodeRepository::new();

    let code = authorization_code();

    let id = code.id();

    repository.save(code.clone()).expect("save should succeed");

    let result = repository.find_by_id(id).expect("lookup should succeed");

    assert_eq!(result, Some(code));
}

#[test]
fn returns_none_when_code_missing() {
    let repository = MemoryAuthorizationCodeRepository::new();

    let result = repository
        .find_by_id(AuthorizationCodeId::new())
        .expect("lookup should succeed");

    assert!(result.is_none());
}
