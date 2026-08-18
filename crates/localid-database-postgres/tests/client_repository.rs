use std::sync::OnceLock;

use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::{Builder, Runtime};

use localid_client::{Client, ClientId, ClientLifecycleState};
use localid_database_postgres::{PostgresClientRepository, migrate};
use localid_repository::ClientRepository;

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

fn repository(pool: PgPool) -> PostgresClientRepository {
    PostgresClientRepository::new(pool, runtime().handle().clone())
}

fn create_client() -> Client {
    Client::new(
        ClientId::new(),
        format!("test-client-{}", ClientId::new()),
        "Test Application",
    )
}

#[tokio::test(flavor = "multi_thread")]
async fn save_and_find_client() {
    let pool = create_pool().await;

    let mut repository = repository(pool);

    let client = create_client();

    let id = client.id();
    let client_id = client.client_id().to_owned();

    repository.save(client).expect("save should succeed");

    let stored = repository
        .find_by_id(id)
        .expect("lookup should succeed")
        .expect("client should exist");

    assert_eq!(stored.id(), id);
    assert_eq!(stored.client_id(), client_id);
    assert_eq!(stored.name(), "Test Application");
}

#[tokio::test(flavor = "multi_thread")]
async fn find_by_client_id_should_return_client() {
    let pool = create_pool().await;

    let mut repository = repository(pool);

    let client = create_client();

    let client_id = client.client_id().to_owned();

    repository.save(client).expect("save should succeed");

    let stored = repository
        .find_by_client_id(&client_id)
        .expect("lookup should succeed")
        .expect("client should exist");

    assert_eq!(stored.client_id(), client_id);
}

#[tokio::test(flavor = "multi_thread")]
async fn find_unknown_client_should_return_none() {
    let pool = create_pool().await;

    let repository = repository(pool);

    let result = repository
        .find_by_id(ClientId::new())
        .expect("lookup should succeed");

    assert!(result.is_none());
}

#[tokio::test(flavor = "multi_thread")]
async fn save_should_update_existing_client_state() {
    let pool = create_pool().await;

    let mut repository = repository(pool);

    let mut client = create_client();

    let id = client.id();

    repository
        .save(client.clone())
        .expect("initial save should succeed");

    client.disable().expect("disable should succeed");

    repository.save(client).expect("update save should succeed");

    let stored = repository
        .find_by_id(id)
        .expect("lookup should succeed")
        .expect("client should exist");

    assert_eq!(stored.state(), ClientLifecycleState::Disabled);
}
