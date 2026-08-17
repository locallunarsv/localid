use sqlx::{PgPool, postgres::PgPoolOptions};

use localid_database_postgres::migrate;

async fn pool() -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/localid")
        .await
        .expect("database should connect")
}

#[tokio::test(flavor = "multi_thread")]
async fn migration_should_run() {
    let pool = pool().await;

    migrate(&pool).await.expect("migration should succeed");
}
