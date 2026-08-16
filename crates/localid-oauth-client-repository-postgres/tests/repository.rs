use std::sync::OnceLock;

use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::{Builder, Runtime};

use localid_oauth_client_repository_postgres::{PostgresOAuthClientRepository, migrate};

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

fn runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| Builder::new_multi_thread().enable_all().build().unwrap())
}

async fn create_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    migrate(&pool).await.unwrap();

    pool
}

fn repository(pool: PgPool) -> PostgresOAuthClientRepository {
    PostgresOAuthClientRepository::new(pool, runtime().handle().clone())
}
