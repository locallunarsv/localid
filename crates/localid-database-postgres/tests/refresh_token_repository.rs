use std::sync::OnceLock;

use chrono::{TimeDelta, Utc};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::{Builder, Runtime};
use uuid::Uuid;

use localid_database_postgres::{PostgresRefreshTokenRepository, migrate};
use localid_refresh_token::{RefreshToken, RefreshTokenId, RefreshTokenLifecycleState};
use localid_repository::RefreshTokenRepository;
use localid_session::SessionId;

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
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/localid")
        .await
        .expect("database should connect");

    migrate(&pool).await.expect("migration should succeed");

    pool
}

fn repository(pool: PgPool) -> PostgresRefreshTokenRepository {
    PostgresRefreshTokenRepository::new(pool, runtime().handle().clone())
}

fn create_refresh_token() -> RefreshToken {
    let created_at = Utc::now();
    let expires_at = created_at + TimeDelta::days(30);

    RefreshToken::new(
        RefreshTokenId::new(),
        SessionId::new(),
        format!("refresh-secret-hash-{}", Uuid::now_v7()),
        created_at,
        expires_at,
    )
    .expect("refresh token should be valid")
}

#[tokio::test(flavor = "multi_thread")]
async fn save_and_find_refresh_token() {
    let pool = create_pool().await;

    let mut repository = repository(pool);

    let token = create_refresh_token();

    let token_id = token.id();

    repository.save(token).expect("save should succeed");

    let stored = repository
        .find_by_id(token_id)
        .expect("lookup should succeed")
        .expect("refresh token should exist");

    assert_eq!(stored.id(), token_id);
    assert_eq!(stored.lifecycle_state(), RefreshTokenLifecycleState::Active);
}

#[tokio::test(flavor = "multi_thread")]
async fn find_by_secret_hash_should_return_refresh_token() {
    let pool = create_pool().await;

    let mut repository = repository(pool);

    let token = create_refresh_token();

    let secret_hash = token.secret_hash().to_string();

    repository.save(token).expect("save should succeed");

    let stored = repository
        .find_by_secret_hash(&secret_hash)
        .expect("lookup should succeed")
        .expect("refresh token should exist");

    assert_eq!(stored.secret_hash(), secret_hash);
}

#[tokio::test(flavor = "multi_thread")]
async fn save_should_update_revoked_refresh_token_state() {
    let pool = create_pool().await;

    let mut repository = repository(pool);

    let mut token = create_refresh_token();

    let token_id = token.id();

    repository
        .save(token.clone())
        .expect("initial save should succeed");

    token.revoke();

    repository.save(token).expect("update save should succeed");

    let stored = repository
        .find_by_id(token_id)
        .expect("lookup should succeed")
        .expect("refresh token should exist");

    assert_eq!(
        stored.lifecycle_state(),
        RefreshTokenLifecycleState::Revoked
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn find_unknown_refresh_token_should_return_none() {
    let pool = create_pool().await;

    let repository = repository(pool);

    let result = repository
        .find_by_id(RefreshTokenId::new())
        .expect("lookup should succeed");

    assert!(result.is_none());
}
