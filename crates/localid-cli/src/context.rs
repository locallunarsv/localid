use std::process;

use localid_config::DatabaseConfig;
use localid_database_postgres::{
    PostgresCredentialRepository, PostgresIdentityRepository, PostgresOAuthClientRepository,
};
use tokio::runtime::Handle;

/// Returns LocalID database configuration.
pub fn database_config() -> DatabaseConfig {
    DatabaseConfig::from_env().unwrap_or_else(|error| {
        eprintln!("Failed to read LOCALID_DATABASE_URL: {error}");
        std::process::exit(1);
    })
}

/// Creates PostgreSQL Identity repository.
pub async fn identity_repository() -> PostgresIdentityRepository {
    let database = database_config();

    match PostgresIdentityRepository::connect(&database, Handle::current()).await {
        Ok(repository) => repository,

        Err(error) => {
            eprintln!("Failed to initialize PostgreSQL identity repository: {error:?}");
            process::exit(1);
        }
    }
}

/// Creates PostgreSQL Credential repository.
pub async fn credential_repository() -> PostgresCredentialRepository {
    let database = database_config();

    match PostgresCredentialRepository::connect(&database, Handle::current()).await {
        Ok(repository) => repository,

        Err(error) => {
            eprintln!("Failed to initialize PostgreSQL credential repository: {error:?}");
            process::exit(1);
        }
    }
}

/// Creates PostgreSQL OAuth client repository.
pub async fn oauth_client_repository() -> PostgresOAuthClientRepository {
    let database = database_config();

    match PostgresOAuthClientRepository::connect(&database, Handle::current()).await {
        Ok(repository) => repository,

        Err(error) => {
            eprintln!("Failed to initialize PostgreSQL OAuth client repository: {error:?}");
            process::exit(1);
        }
    }
}
