mod common;

use sqlx::{PgPool, postgres::PgPoolOptions};

use localid_database_postgres::migrate;

use common::test_database;

async fn pool() -> PgPool {
    let database = test_database();

    PgPoolOptions::new()
        .max_connections(database.max_connections())
        .connect(database.url())
        .await
        .expect("test database should connect")
}

#[tokio::test(flavor = "multi_thread")]
async fn migration_should_run() {
    let pool = pool().await;

    migrate(&pool).await.expect("migration should succeed");
}
