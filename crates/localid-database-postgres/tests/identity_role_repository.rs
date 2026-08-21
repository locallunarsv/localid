mod common;

use std::sync::OnceLock;

use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::{Builder, Runtime};
use uuid::Uuid;

use localid_database_postgres::{PostgresIdentityRoleRepository, migrate};
use localid_identity::IdentityId;
use localid_repository::IdentityRoleRepository;

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

fn repository(pool: PgPool) -> PostgresIdentityRoleRepository {
    PostgresIdentityRoleRepository::new(pool, runtime().handle().clone())
}

async fn cleanup(pool: &PgPool) {
    sqlx::query(
        r#"
        TRUNCATE TABLE
            identity_roles,
            role_permissions,
            permissions,
            roles
        CASCADE;
        "#,
    )
    .execute(pool)
    .await
    .expect("truncate should succeed");
}

async fn seed_role(pool: &PgPool, identity_id: IdentityId) {
    cleanup(pool).await;

    let role_id = Uuid::now_v7();
    let permission_read_id = Uuid::now_v7();
    let permission_write_id = Uuid::now_v7();

    sqlx::query(
        r#"
        INSERT INTO roles (
            id,
            name
        )
        VALUES ($1, $2)
        "#,
    )
    .bind(role_id)
    .bind("admin")
    .execute(pool)
    .await
    .expect("role insert should succeed");

    sqlx::query(
        r#"
        INSERT INTO permissions (
            id,
            name
        )
        VALUES
            ($1, $2),
            ($3, $4)
        "#,
    )
    .bind(permission_read_id)
    .bind("user.read")
    .bind(permission_write_id)
    .bind("user.write")
    .execute(pool)
    .await
    .expect("permission insert should succeed");

    sqlx::query(
        r#"
        INSERT INTO role_permissions (
            role_id,
            permission_id
        )
        VALUES
            ($1, $2),
            ($1, $3)
        "#,
    )
    .bind(role_id)
    .bind(permission_read_id)
    .bind(permission_write_id)
    .execute(pool)
    .await
    .expect("role permission insert should succeed");

    sqlx::query(
        r#"
        INSERT INTO identities (
            id,
            lifecycle_state
        )
        VALUES ($1, $2)
        "#,
    )
    .bind(identity_id.as_uuid())
    .bind("active")
    .execute(pool)
    .await
    .expect("identity insert should succeed");

    sqlx::query(
        r#"
        INSERT INTO identity_roles (
            identity_id,
            role_id
        )
        VALUES ($1, $2)
        "#,
    )
    .bind(identity_id.as_uuid())
    .bind(role_id)
    .execute(pool)
    .await
    .expect("identity role insert should succeed");
}

#[tokio::test(flavor = "multi_thread")]
async fn find_roles_should_return_roles_and_permissions() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;

    let identity_id = IdentityId::new();

    seed_role(&pool, identity_id).await;

    let repository = repository(pool);

    let roles = repository
        .find_roles(identity_id)
        .expect("find roles should succeed");

    assert_eq!(roles.len(), 1);

    let role = &roles[0];

    assert_eq!(role.name(), "admin");
    assert_eq!(role.permissions().len(), 2);

    assert!(
        role.permissions()
            .iter()
            .any(|permission| permission.name() == "user.read")
    );

    assert!(
        role.permissions()
            .iter()
            .any(|permission| permission.name() == "user.write")
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn find_roles_should_return_empty_when_identity_has_no_role() {
    let _guard = test_lock().lock().await;

    let pool = create_pool().await;

    cleanup(&pool).await;

    let repository = repository(pool);

    let roles = repository
        .find_roles(IdentityId::new())
        .expect("find roles should succeed");

    assert!(roles.is_empty());
}
