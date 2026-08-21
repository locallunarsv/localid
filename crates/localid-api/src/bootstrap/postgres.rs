//! PostgreSQL bootstrap helpers.

use tokio::runtime::Handle;

use localid_config::DatabaseConfig;

use localid_database_postgres::{
    connect, migrate, DatabaseError, PostgresAuthorizationCodeRepository, PostgresClientRepository,
    PostgresCredentialRepository, PostgresIdentityRepository, PostgresIdentityRoleRepository,
    PostgresOAuthClientRepository, PostgresPasswordMaterialRepository,
    PostgresRefreshTokenRepository, PostgresSessionRepository, PostgresTokenRepository,
};

use super::repository::SharedRepository;

pub type SharedPostgresAuthorizationCodeRepository =
    SharedRepository<PostgresAuthorizationCodeRepository>;

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
    /// Authorization code repository.
    pub authorization_code: SharedPostgresAuthorizationCodeRepository,

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
    let pool = connect(config).await?;

    migrate(&pool)
        .await
        .map_err(|_| DatabaseError::InvalidData)?;

    let authorization_code =
        PostgresAuthorizationCodeRepository::new(pool.clone(), runtime.clone());

    let client = PostgresClientRepository::new(pool.clone(), runtime.clone());

    let credential = PostgresCredentialRepository::new(pool.clone(), runtime.clone());

    let identity = PostgresIdentityRepository::new(pool.clone(), runtime.clone());

    let identity_role = PostgresIdentityRoleRepository::new(pool.clone(), runtime.clone());

    let oauth_client = PostgresOAuthClientRepository::new(pool.clone(), runtime.clone());

    let password_material = PostgresPasswordMaterialRepository::new(pool.clone(), runtime.clone());

    let refresh_token = PostgresRefreshTokenRepository::new(pool.clone(), runtime.clone());

    let session = PostgresSessionRepository::new(pool.clone(), runtime.clone());

    let token = PostgresTokenRepository::new(pool, runtime);

    Ok(PostgresRepositories {
        authorization_code: SharedRepository::new(authorization_code),
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
