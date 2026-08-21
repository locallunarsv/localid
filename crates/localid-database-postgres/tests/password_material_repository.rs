mod common;

use std::sync::OnceLock;

use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::{Builder, Runtime};

use localid_credential::{Credential, CredentialId, CredentialKind};
use localid_database_postgres::{
    PostgresCredentialRepository, PostgresPasswordMaterialRepository, migrate,
};
use localid_identity::IdentityId;
use localid_password::{PasswordHash, PasswordMaterial};
use localid_repository::{CredentialRepository, PasswordMaterialRepository};

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

fn repository(pool: PgPool) -> PostgresPasswordMaterialRepository {
    PostgresPasswordMaterialRepository::new(pool, runtime().handle().clone())
}

fn credential_repository(pool: PgPool) -> PostgresCredentialRepository {
    PostgresCredentialRepository::new(pool, runtime().handle().clone())
}

fn create_credential() -> Credential {
    Credential::new(
        CredentialId::new(),
        IdentityId::new(),
        CredentialKind::Password,
    )
}

fn create_material(credential_id: CredentialId) -> PasswordMaterial {
    PasswordMaterial::new(
        credential_id,
        PasswordHash::new("$argon2id$example-hash".to_string()),
    )
}

#[tokio::test(flavor = "multi_thread")]
async fn save_and_find_password_material() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;

    let mut credential_repo = credential_repository(pool.clone());
    let mut repository = repository(pool);

    let credential = create_credential();
    let credential_id = credential.id();

    credential_repo
        .save(credential)
        .expect("credential save should succeed");

    let material = create_material(credential_id);

    repository.save(material).expect("save should succeed");

    let stored = repository
        .find_by_credential_id(credential_id)
        .expect("lookup should succeed")
        .expect("material should exist");

    assert_eq!(stored.credential_id(), credential_id);
}

#[tokio::test(flavor = "multi_thread")]
async fn save_should_replace_existing_hash() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;

    let mut credential_repo = credential_repository(pool.clone());
    let mut repository = repository(pool);

    let credential = create_credential();
    let credential_id = credential.id();

    credential_repo
        .save(credential)
        .expect("credential save should succeed");

    let first = PasswordMaterial::new(credential_id, PasswordHash::new("first-hash".to_string()));

    repository.save(first).expect("initial save should succeed");

    let second = PasswordMaterial::new(credential_id, PasswordHash::new("second-hash".to_string()));

    repository.save(second).expect("update save should succeed");

    let stored = repository
        .find_by_credential_id(credential_id)
        .expect("lookup should succeed")
        .expect("material should exist");

    assert_eq!(stored.password_hash().as_str(), "second-hash");
}

#[tokio::test(flavor = "multi_thread")]
async fn find_unknown_password_material_should_return_none() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;
    let repository = repository(pool);

    let result = repository
        .find_by_credential_id(CredentialId::new())
        .expect("lookup should succeed");

    assert!(result.is_none());
}
