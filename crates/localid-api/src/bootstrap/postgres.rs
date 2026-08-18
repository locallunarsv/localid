//! PostgreSQL bootstrap helpers.

use tokio::runtime::Handle;

use localid_config::DatabaseConfig;

use localid_database_postgres::{
    migrate, DatabaseError, PostgresClientRepository, PostgresCredentialRepository,
    PostgresIdentityRepository, PostgresIdentityRoleRepository, PostgresOAuthClientRepository,
    PostgresPasswordMaterialRepository, PostgresRefreshTokenRepository, PostgresSessionRepository,
    PostgresTokenRepository,
};

use super::repository::SharedRepository;

pub type SharedPostgresClientRepository = SharedRepository<PostgresClientRepository>;

pub type SharedPostgresCredentialRepository = SharedRepository<PostgresCredentialRepository>;

pub type SharedPostgresIdentityRepository = SharedRepository<PostgresIdentityRepository>;

pub type SharedPostgresIdentityRoleRepository = SharedRepository<PostgresIdentityRoleRepository>;

pub type SharedPostgresOAuthClientRepository = SharedRepository<PostgresOAuthClientRepository>;

pub type SharedPostgresPasswordMaterialRepository =
    SharedRepository<PostgresPasswordMaterialRepository>;

pub type SharedPostgresRefreshTokenRepository = SharedRepository<PostgresRefreshTokenRepository>;

pub type SharedPostgresSessionRepository = SharedRepository<PostgresSessionRepository>;

pub type SharedPostgresTokenRepository = SharedRepository<PostgresTokenRepository>;

/// Collection of PostgreSQL repositories required by the application.
pub struct PostgresRepositories {
    /// Client repository.
    pub client: SharedPostgresClientRepository,

    /// Credential repository.
    pub credential: SharedPostgresCredentialRepository,

    /// Identity repository.
    pub identity: SharedPostgresIdentityRepository,

    /// Identity role repository.
    pub identity_role: SharedPostgresIdentityRoleRepository,

    /// OAuth client repository.
    pub oauth_client: SharedPostgresOAuthClientRepository,

    /// Password material repository.
    pub password_material: SharedPostgresPasswordMaterialRepository,

    /// Refresh token repository.
    pub refresh_token: SharedPostgresRefreshTokenRepository,

    /// Session repository.
    pub session: SharedPostgresSessionRepository,

    /// Token repository.
    pub token: SharedPostgresTokenRepository,
}

/// Creates all PostgreSQL repositories.
pub async fn create_postgres_repositories(
    config: &DatabaseConfig,
    runtime: Handle,
) -> Result<PostgresRepositories, DatabaseError> {
    let client = PostgresClientRepository::connect(config, runtime.clone()).await?;

    let credential = PostgresCredentialRepository::connect(config, runtime.clone()).await?;

    let identity = PostgresIdentityRepository::connect(config, runtime.clone()).await?;

    let identity_role = PostgresIdentityRoleRepository::connect(config, runtime.clone()).await?;

    let oauth_client = PostgresOAuthClientRepository::connect(config, runtime.clone()).await?;

    let password_material =
        PostgresPasswordMaterialRepository::connect(config, runtime.clone()).await?;

    let refresh_token = PostgresRefreshTokenRepository::connect(config, runtime.clone()).await?;

    let session = PostgresSessionRepository::connect(config, runtime.clone()).await?;

    let token = PostgresTokenRepository::connect(config, runtime).await?;

    migrate(oauth_client.pool())
        .await
        .map_err(|_| DatabaseError::InvalidData)?;

    Ok(PostgresRepositories {
        client: SharedRepository::new(client),
        credential: SharedRepository::new(credential),
        identity: SharedRepository::new(identity),
        identity_role: SharedRepository::new(identity_role),
        oauth_client: SharedRepository::new(oauth_client),
        password_material: SharedRepository::new(password_material),
        refresh_token: SharedRepository::new(refresh_token),
        session: SharedRepository::new(session),
        token: SharedRepository::new(token),
    })
}

/// Creates PostgreSQL OAuth client repository.
pub async fn create_postgres_oauth_client_repository(
    config: &DatabaseConfig,
    runtime: Handle,
) -> Result<SharedPostgresOAuthClientRepository, DatabaseError> {
    let repositories = create_postgres_repositories(config, runtime).await?;

    Ok(repositories.oauth_client)
}
