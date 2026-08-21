mod common;

use std::sync::OnceLock;

use chrono::{TimeDelta, Utc};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::{Builder, Runtime};

use localid_client::ClientId;
use localid_database_postgres::{PostgresSessionRepository, migrate};
use localid_identity::IdentityId;
use localid_repository::SessionRepository;
use localid_session::{Session, SessionId, SessionLifecycleState};

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

fn repository(pool: PgPool) -> PostgresSessionRepository {
    PostgresSessionRepository::new(pool, runtime().handle().clone())
}

fn create_session(identity_id: IdentityId) -> Session {
    let created_at = Utc::now();
    let expires_at = created_at + TimeDelta::hours(1);

    Session::new(
        SessionId::new(),
        identity_id,
        ClientId::new(),
        created_at,
        expires_at,
    )
    .expect("session should be valid")
}

#[tokio::test(flavor = "multi_thread")]
async fn save_and_find_session() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;
    let mut repository = repository(pool);

    let identity_id = IdentityId::new();
    let session = create_session(identity_id);
    let session_id = session.id();

    repository.save(session).expect("save should succeed");

    let stored = repository
        .find_by_id(session_id)
        .expect("lookup should succeed")
        .expect("session should exist");

    assert_eq!(stored.id(), session_id);
    assert_eq!(stored.identity_id(), identity_id);
    assert_eq!(stored.lifecycle_state(), SessionLifecycleState::Active);
}

#[tokio::test(flavor = "multi_thread")]
async fn save_should_update_revoked_session_state() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;
    let mut repository = repository(pool);

    let identity_id = IdentityId::new();
    let mut session = create_session(identity_id);
    let session_id = session.id();

    repository
        .save(session.clone())
        .expect("initial save should succeed");

    session.revoke();

    repository
        .save(session)
        .expect("update save should succeed");

    let stored = repository
        .find_by_id(session_id)
        .expect("lookup should succeed")
        .expect("session should exist");

    assert_eq!(stored.lifecycle_state(), SessionLifecycleState::Revoked);
}

#[tokio::test(flavor = "multi_thread")]
async fn find_by_identity_id_should_return_sessions() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;
    let mut repository = repository(pool);

    let identity_id = IdentityId::new();

    repository
        .save(create_session(identity_id))
        .expect("first save should succeed");

    repository
        .save(create_session(identity_id))
        .expect("second save should succeed");

    let sessions = repository
        .find_by_identity_id(identity_id)
        .expect("lookup should succeed");

    assert_eq!(sessions.len(), 2);
}

#[tokio::test(flavor = "multi_thread")]
async fn find_unknown_session_should_return_none() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;
    let repository = repository(pool);

    let result = repository
        .find_by_id(SessionId::new())
        .expect("lookup should succeed");

    assert!(result.is_none());
}
