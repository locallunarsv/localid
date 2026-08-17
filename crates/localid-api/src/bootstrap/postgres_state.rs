//! PostgreSQL application bootstrap.

use tokio::runtime::Handle;

use localid_config::DatabaseConfig;
use localid_database_postgres::DatabaseError;

use super::{create_postgres_oauth_client_repository, SharedPostgresOAuthClientRepository};

/// Creates PostgreSQL OAuth client repository.
pub async fn create_postgres_repository(
    database: DatabaseConfig,
    runtime: Handle,
) -> Result<SharedPostgresOAuthClientRepository, sqlx::Error> {
    let repository = create_postgres_oauth_client_repository(&database, runtime)
        .await
        .map_err(|error| match error {
            DatabaseError::Connection(error) => error,

            _ => sqlx::Error::Protocol("invalid postgres repository data".into()),
        })?;

    Ok(repository)
}
