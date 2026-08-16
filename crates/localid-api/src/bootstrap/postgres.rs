//! PostgreSQL bootstrap helpers.

use tokio::runtime::Handle;

use localid_config::DatabaseConfig;
use localid_oauth_client_repository_postgres::{
    migrate, PostgresOAuthClientRepository, PostgresOAuthClientRepositoryError,
};

use super::{repository::SharedRepository, SharedPostgresOAuthClientRepository};

/// Creates PostgreSQL OAuth client repository.
pub async fn create_postgres_oauth_client_repository(
    config: &DatabaseConfig,
    runtime: Handle,
) -> Result<SharedPostgresOAuthClientRepository, PostgresOAuthClientRepositoryError> {
    let repository = PostgresOAuthClientRepository::connect(config, runtime).await?;

    migrate(repository.pool())
        .await
        .map_err(|_| PostgresOAuthClientRepositoryError::InvalidData)?;

    Ok(SharedRepository::new(repository))
}
