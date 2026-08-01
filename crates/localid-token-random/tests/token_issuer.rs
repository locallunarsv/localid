use chrono::{TimeDelta, Utc};
use localid_session::SessionId;
use localid_token::TokenIssuer;
use localid_token_random::RandomTokenIssuer;

#[test]
fn issues_token_with_secret() {
    let issuer = RandomTokenIssuer::new();

    let expires_at = Utc::now() + TimeDelta::hours(1);

    let issued = issuer
        .issue(SessionId::new(), expires_at)
        .expect("token issuance should succeed");

    assert!(!issued.secret().is_empty());
    assert!(!issued.token().secret_hash().is_empty());
}

#[test]
fn issues_unique_secrets() {
    let issuer = RandomTokenIssuer::new();

    let expires_at = Utc::now() + TimeDelta::hours(1);

    let first = issuer
        .issue(SessionId::new(), expires_at)
        .expect("first token should be issued");

    let second = issuer
        .issue(SessionId::new(), expires_at)
        .expect("second token should be issued");

    assert_ne!(first.secret(), second.secret());

    assert_ne!(first.token().id(), second.token().id());
}
