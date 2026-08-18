use std::sync::OnceLock;

use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::{Builder, Runtime};

use localid_credential::{Credential, CredentialId, CredentialKind, CredentialLifecycleState};
use localid_database_postgres::{PostgresCredentialRepository, migrate};
use localid_identity::IdentityId;
use localid_repository::CredentialRepository;

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

fn repository(pool: PgPool) -> PostgresCredentialRepository {
    PostgresCredentialRepository::new(pool, runtime().handle().clone())
}

fn create_credential(identity_id: IdentityId) -> Credential {
    Credential::new(CredentialId::new(), identity_id, CredentialKind::Password)
}

#[tokio::test(flavor = "multi_thread")]
async fn save_and_find_credential() {
    let pool = create_pool().await;

    let mut repository = repository(pool);

    let identity_id = IdentityId::new();

    let credential = create_credential(identity_id);

    let credential_id = credential.id();

    repository.save(credential).expect("save should succeed");

    let stored = repository
        .find_by_id(credential_id)
        .expect("lookup should succeed")
        .expect("credential should exist");

    assert_eq!(stored.id(), credential_id);
    assert_eq!(stored.identity_id(), identity_id);
    assert_eq!(stored.kind(), CredentialKind::Password);
    assert_eq!(stored.lifecycle_state(), CredentialLifecycleState::Active);
}

#[tokio::test(flavor = "multi_thread")]
async fn save_should_update_existing_credential_state() {
    let pool = create_pool().await;

    let mut repository = repository(pool);

    let identity_id = IdentityId::new();

    let mut credential = create_credential(identity_id);

    let credential_id = credential.id();

    repository
        .save(credential.clone())
        .expect("initial save should succeed");

    credential.disable().expect("credential should disable");

    repository
        .save(credential)
        .expect("update save should succeed");

    let stored = repository
        .find_by_id(credential_id)
        .expect("lookup should succeed")
        .expect("credential should exist");

    assert_eq!(stored.lifecycle_state(), CredentialLifecycleState::Disabled);
}

#[tokio::test(flavor = "multi_thread")]
async fn find_by_identity_id_should_return_credentials() {
    let pool = create_pool().await;

    let mut repository = repository(pool);

    let identity_id = IdentityId::new();

    repository
        .save(create_credential(identity_id))
        .expect("first save should succeed");

    repository
        .save(create_credential(identity_id))
        .expect("second save should succeed");

    let credentials = repository
        .find_by_identity_id(identity_id)
        .expect("lookup should succeed");

    assert_eq!(credentials.len(), 2);
}

#[tokio::test(flavor = "multi_thread")]
async fn find_unknown_credential_should_return_none() {
    let pool = create_pool().await;

    let repository = repository(pool);

    let result = repository
        .find_by_id(CredentialId::new())
        .expect("lookup should succeed");

    assert!(result.is_none());
}
