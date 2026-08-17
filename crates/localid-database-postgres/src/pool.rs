use sqlx::{PgPool, postgres::PgPoolOptions};

use localid_config::DatabaseConfig;

use crate::DatabaseError;

/// Creates PostgreSQL connection pool.
pub async fn connect(config: &DatabaseConfig) -> Result<PgPool, DatabaseError> {
    PgPoolOptions::new()
        .max_connections(config.max_connections())
        .connect(config.url())
        .await
        .map_err(DatabaseError::Connection)
}
