use chrono::{TimeDelta, Utc};
use localid_refresh_token_random::{RandomRefreshTokenIssuer, RefreshTokenIssuer};
use localid_session::SessionId;

#[test]
fn issues_refresh_token_with_secret() {
    let issuer = RandomRefreshTokenIssuer::new();

    let issued = issuer
        .issue(SessionId::new(), Utc::now() + TimeDelta::days(30))
        .expect("refresh token should be issued");

    assert!(!issued.secret().is_empty());
    assert!(!issued.token().secret_hash().is_empty());
}

#[test]
fn issues_unique_refresh_tokens() {
    let issuer = RandomRefreshTokenIssuer::new();

    let expires_at = Utc::now() + TimeDelta::days(30);

    let first = issuer
        .issue(SessionId::new(), expires_at)
        .expect("first refresh token should be issued");

    let second = issuer
        .issue(SessionId::new(), expires_at)
        .expect("second refresh token should be issued");

    assert_ne!(first.secret(), second.secret());

    assert_ne!(first.token().id(), second.token().id());
}
