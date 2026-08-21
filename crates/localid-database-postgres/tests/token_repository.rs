mod common;

use std::sync::OnceLock;

use chrono::{TimeDelta, Utc};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::{Builder, Runtime};
use uuid::Uuid;

use localid_database_postgres::{PostgresTokenRepository, migrate};
use localid_repository::TokenRepository;
use localid_session::SessionId;
use localid_token::{Token, TokenId, TokenLifecycleState};

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

fn repository(pool: PgPool) -> PostgresTokenRepository {
    PostgresTokenRepository::new(pool, runtime().handle().clone())
}

fn create_token() -> Token {
    let created_at = Utc::now();
    let expires_at = created_at + TimeDelta::hours(1);

    Token::new(
        TokenId::new(),
        SessionId::new(),
        format!("secret-hash-{}", Uuid::now_v7()),
        created_at,
        expires_at,
    )
    .expect("token should be valid")
}

#[tokio::test(flavor = "multi_thread")]
async fn save_and_find_token() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;
    let mut repository = repository(pool);

    let token = create_token();
    let token_id = token.id();

    repository.save(token).expect("save should succeed");

    let stored = repository
        .find_by_id(token_id)
        .expect("lookup should succeed")
        .expect("token should exist");

    assert_eq!(stored.id(), token_id);
    assert_eq!(stored.lifecycle_state(), TokenLifecycleState::Active);
}

#[tokio::test(flavor = "multi_thread")]
async fn find_by_secret_hash_should_return_token() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;
    let mut repository = repository(pool);

    let token = create_token();
    let secret_hash = token.secret_hash().to_string();

    repository.save(token).expect("save should succeed");

    let stored = repository
        .find_by_secret_hash(&secret_hash)
        .expect("lookup should succeed")
        .expect("token should exist");

    assert_eq!(stored.secret_hash(), secret_hash);
}

#[tokio::test(flavor = "multi_thread")]
async fn save_should_update_revoked_token_state() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;
    let mut repository = repository(pool);

    let mut token = create_token();
    let token_id = token.id();

    repository
        .save(token.clone())
        .expect("initial save should succeed");

    token.revoke();

    repository.save(token).expect("update save should succeed");

    let stored = repository
        .find_by_id(token_id)
        .expect("lookup should succeed")
        .expect("token should exist");

    assert_eq!(stored.lifecycle_state(), TokenLifecycleState::Revoked);
}

#[tokio::test(flavor = "multi_thread")]
async fn find_unknown_token_should_return_none() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;
    let repository = repository(pool);

    let result = repository
        .find_by_id(TokenId::new())
        .expect("lookup should succeed");

    assert!(result.is_none());
}
