mod common;

use std::sync::OnceLock;

use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::{Builder, Runtime};

use localid_database_postgres::{PostgresIdentityRepository, migrate};
use localid_identity::{Identity, IdentityId, LifecycleState};
use localid_repository::IdentityRepository;

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

fn repository(pool: PgPool) -> PostgresIdentityRepository {
    PostgresIdentityRepository::new(pool, runtime().handle().clone())
}

#[tokio::test(flavor = "multi_thread")]
async fn save_and_find_identity() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;
    let mut repository = repository(pool);

    let identity_id = IdentityId::new();
    let identity = Identity::new(identity_id);

    repository.save(identity).expect("save should succeed");

    let stored = repository
        .find_by_id(identity_id)
        .expect("lookup should succeed")
        .expect("identity should exist");

    assert_eq!(stored.id(), identity_id);
    assert_eq!(stored.lifecycle_state(), LifecycleState::Active);
}

#[tokio::test(flavor = "multi_thread")]
async fn save_should_update_existing_identity_state() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;
    let mut repository = repository(pool);

    let identity_id = IdentityId::new();
    let mut identity = Identity::new(identity_id);

    repository
        .save(identity.clone())
        .expect("initial save should succeed");

    identity.disable().expect("identity should disable");

    repository
        .save(identity)
        .expect("update save should succeed");

    let stored = repository
        .find_by_id(identity_id)
        .expect("lookup should succeed")
        .expect("identity should exist");

    assert_eq!(stored.lifecycle_state(), LifecycleState::Disabled);
}

#[tokio::test(flavor = "multi_thread")]
async fn find_unknown_identity_should_return_none() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;
    let repository = repository(pool);

    let result = repository
        .find_by_id(IdentityId::new())
        .expect("lookup should succeed");

    assert!(result.is_none());
}
