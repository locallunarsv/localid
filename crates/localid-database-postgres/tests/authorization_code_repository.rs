mod common;

use std::sync::OnceLock;

use chrono::{TimeDelta, Utc};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::{Builder, Runtime};
use uuid::Uuid;

use localid_database_postgres::{PostgresAuthorizationCodeRepository, migrate};
use localid_identity::IdentityId;
use localid_oauth_authorization::{
    AuthorizationCode, AuthorizationCodeId, AuthorizationCodeLifecycleState,
    AuthorizationCodeRepository, CodeChallengeMethod,
};
use localid_oauth_client::OAuthClientId;

use common::{test_database, test_lock};

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

fn runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| {
        Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("runtime should build")
    })
}

async fn create_pool() -> PgPool {
    let database = test_database();

    let pool = PgPoolOptions::new()
        .max_connections(database.max_connections())
        .connect(database.url())
        .await
        .expect("test database should connect");

    migrate(&pool).await.expect("migration should succeed");

    pool
}

fn repository(pool: PgPool) -> PostgresAuthorizationCodeRepository {
    PostgresAuthorizationCodeRepository::new(pool, runtime().handle().clone())
}

async fn clear(pool: &PgPool) {
    sqlx::query("TRUNCATE TABLE authorization_codes")
        .execute(pool)
        .await
        .expect("authorization codes should clear");
}

fn create_authorization_code() -> AuthorizationCode {
    let created_at = Utc::now();
    let expires_at = created_at + TimeDelta::minutes(10);

    AuthorizationCode::new_with_nonce_and_pkce(
        AuthorizationCodeId::new(),
        OAuthClientId::new(),
        IdentityId::new(),
        format!("authorization-code-hash-{}", Uuid::now_v7()),
        "http://localhost:3000/callback",
        Some("test-nonce".to_owned()),
        vec!["openid".to_owned(), "profile".to_owned()],
        Some("request-state".to_owned()),
        Some("pkce-challenge".to_owned()),
        Some(CodeChallengeMethod::S256),
        created_at,
        expires_at,
    )
    .expect("authorization code should be valid")
}

#[tokio::test(flavor = "multi_thread")]
async fn save_and_find_authorization_code() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;
    clear(&pool).await;

    let mut repository = repository(pool);

    let code = create_authorization_code();
    let code_id = code.id();

    repository.save(code).expect("save should succeed");

    let stored = repository
        .find_by_id(code_id)
        .expect("lookup should succeed")
        .expect("authorization code should exist");

    assert_eq!(stored.id(), code_id);
    assert_eq!(stored.state(), AuthorizationCodeLifecycleState::Active);
    assert_eq!(stored.redirect_uri(), "http://localhost:3000/callback");
    assert_eq!(stored.nonce(), Some("test-nonce"));
    assert_eq!(stored.scope(), &["openid".to_owned(), "profile".to_owned()]);
    assert_eq!(stored.request_state(), Some("request-state"));
    assert_eq!(stored.pkce_challenge(), Some("pkce-challenge"));
    assert_eq!(stored.pkce_method(), Some(&CodeChallengeMethod::S256));
}

#[tokio::test(flavor = "multi_thread")]
async fn find_by_hash_should_return_authorization_code() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;
    clear(&pool).await;

    let mut repository = repository(pool);

    let code = create_authorization_code();
    let code_hash = code.code_hash().to_owned();

    repository.save(code).expect("save should succeed");

    let stored = repository
        .find_by_hash(&code_hash)
        .expect("lookup should succeed")
        .expect("authorization code should exist");

    assert_eq!(stored.code_hash(), code_hash);
}

#[tokio::test(flavor = "multi_thread")]
async fn save_should_update_consumed_authorization_code_state() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;
    clear(&pool).await;

    let mut repository = repository(pool);

    let mut code = create_authorization_code();
    let code_id = code.id();

    repository
        .save(code.clone())
        .expect("initial save should succeed");

    code.consume().expect("authorization code should consume");

    repository.save(code).expect("update save should succeed");

    let stored = repository
        .find_by_id(code_id)
        .expect("lookup should succeed")
        .expect("authorization code should exist");

    assert_eq!(stored.state(), AuthorizationCodeLifecycleState::Consumed);
}

#[tokio::test(flavor = "multi_thread")]
async fn find_unknown_authorization_code_should_return_none() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;
    clear(&pool).await;

    let repository = repository(pool);

    let result = repository
        .find_by_id(AuthorizationCodeId::new())
        .expect("lookup should succeed");

    assert!(result.is_none());
}

#[tokio::test(flavor = "multi_thread")]
async fn find_unknown_authorization_code_hash_should_return_none() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;
    clear(&pool).await;

    let repository = repository(pool);

    let result = repository
        .find_by_hash("missing-authorization-code-hash")
        .expect("lookup should succeed");

    assert!(result.is_none());
}
