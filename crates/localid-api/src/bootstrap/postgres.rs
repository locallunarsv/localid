//! PostgreSQL bootstrap helpers.

use tokio::runtime::Handle;

use localid_config::DatabaseConfig;
use localid_database_postgres::{migrate, DatabaseError, PostgresOAuthClientRepository};

use super::{repository::SharedRepository, SharedPostgresOAuthClientRepository};

/// Creates PostgreSQL OAuth client repository.
pub async fn create_postgres_oauth_client_repository(
    config: &DatabaseConfig,
    runtime: Handle,
) -> Result<SharedPostgresOAuthClientRepository, DatabaseError> {
    let repository = PostgresOAuthClientRepository::connect(config, runtime).await?;

    migrate(repository.pool())
        .await
        .map_err(|_| DatabaseError::InvalidData)?;

    Ok(SharedRepository::new(repository))
}
