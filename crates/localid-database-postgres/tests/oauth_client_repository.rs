use std::sync::OnceLock;

use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::{Builder, Runtime};
use tokio::sync::Mutex;

use localid_client::ClientId;
use localid_database_postgres::{PostgresOAuthClientRepository, migrate};
use localid_oauth_client::{
    OAuthClient, OAuthClientId, OAuthClientLifecycleState, OAuthClientRepository,
};

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| {
        Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("runtime should build")
    })
}

fn test_lock() -> &'static Mutex<()> {
    TEST_LOCK.get_or_init(|| Mutex::new(()))
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

fn repository(pool: PgPool) -> PostgresOAuthClientRepository {
    PostgresOAuthClientRepository::new(pool, runtime().handle().clone())
}

fn create_client_with_client_id(client_id: &str, name: &str) -> OAuthClient {
    OAuthClient::new(
        OAuthClientId::new(),
        ClientId::new(),
        client_id.to_string(),
        name.to_string(),
        "secret-hash".to_string(),
        vec!["http://localhost:3000/callback".to_string()],
    )
}

fn create_client() -> OAuthClient {
    create_client_with_client_id("test-client", "Test Client")
}

#[tokio::test(flavor = "multi_thread")]
async fn save_and_find_oauth_client() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;

    let mut repository = repository(pool);

    repository.clear().expect("clear should succeed");

    let client = create_client();

    let client_id = client.id();

    repository.save(client).expect("save should succeed");

    let stored = repository
        .find_by_id(client_id)
        .expect("lookup should succeed")
        .expect("client should exist");

    assert_eq!(stored.id(), client_id);
    assert_eq!(stored.state(), OAuthClientLifecycleState::Active);
}

#[tokio::test(flavor = "multi_thread")]
async fn find_by_client_id_should_return_client() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;

    let mut repository = repository(pool);

    repository.clear().expect("clear should succeed");

    let client = create_client();

    let public_client_id = client.client_id().to_string();

    repository.save(client).expect("save should succeed");

    let stored = repository
        .find_by_client_id(&public_client_id)
        .expect("lookup should succeed")
        .expect("client should exist");

    assert_eq!(stored.client_id(), public_client_id);
}

#[tokio::test(flavor = "multi_thread")]
async fn save_should_update_existing_client_state() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;

    let mut repository = repository(pool);

    repository.clear().expect("clear should succeed");

    let mut client = create_client();

    let client_id = client.id();

    repository
        .save(client.clone())
        .expect("initial save should succeed");

    client.disable().expect("client should disable");

    repository.save(client).expect("update save should succeed");

    let stored = repository
        .find_by_id(client_id)
        .expect("lookup should succeed")
        .expect("client should exist");

    assert_eq!(stored.state(), OAuthClientLifecycleState::Disabled);
}

#[tokio::test(flavor = "multi_thread")]
async fn find_unknown_client_should_return_none() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;

    let repository = repository(pool);

    repository.clear().expect("clear should succeed");

    let result = repository
        .find_by_id(OAuthClientId::new())
        .expect("lookup should succeed");

    assert!(result.is_none());
}

#[tokio::test(flavor = "multi_thread")]
async fn find_all_should_return_clients() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;

    let mut repository = repository(pool);

    repository.clear().expect("clear should succeed");

    let first = create_client_with_client_id("first-client", "First Client");

    let second = create_client_with_client_id("second-client", "Second Client");

    repository.save(first).expect("first save should succeed");

    repository.save(second).expect("second save should succeed");

    let clients = repository.find_all().expect("find all should succeed");

    assert_eq!(clients.len(), 2);
}
