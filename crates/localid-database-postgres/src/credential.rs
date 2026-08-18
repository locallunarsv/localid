use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::Handle;
use uuid::Uuid;

use localid_config::DatabaseConfig;
use localid_credential::{Credential, CredentialId, CredentialKind, CredentialLifecycleState};
use localid_identity::IdentityId;
use localid_repository::CredentialRepository;

use crate::DatabaseError;

#[derive(Debug, sqlx::FromRow)]
struct CredentialRow {
    id: Uuid,
    identity_id: Uuid,
    kind: String,
    lifecycle_state: String,
}

/// PostgreSQL repository for Credential aggregates.
#[derive(Clone)]
pub struct PostgresCredentialRepository {
    pool: PgPool,
    runtime: Handle,
}

impl PostgresCredentialRepository {
    /// Creates repository from PostgreSQL pool.
    #[must_use]
    pub fn new(pool: PgPool, runtime: Handle) -> Self {
        Self { pool, runtime }
    }

    /// Returns PostgreSQL pool.
    #[must_use]
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Creates repository by connecting to PostgreSQL.
    pub async fn connect(config: &DatabaseConfig, runtime: Handle) -> Result<Self, DatabaseError> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections())
            .connect(config.url())
            .await
            .map_err(DatabaseError::Connection)?;

        Ok(Self::new(pool, runtime))
    }

    fn block_on<F, T>(&self, future: F) -> T
    where
        F: std::future::Future<Output = T>,
    {
        tokio::task::block_in_place(|| self.runtime.block_on(future))
    }

    fn map_kind(value: &str) -> Result<CredentialKind, DatabaseError> {
        match value {
            "password" => Ok(CredentialKind::Password),
            "passkey" => Ok(CredentialKind::Passkey),
            "api_key" => Ok(CredentialKind::ApiKey),
            _ => Err(DatabaseError::InvalidData),
        }
    }

    fn map_state(value: &str) -> Result<CredentialLifecycleState, DatabaseError> {
        match value {
            "active" => Ok(CredentialLifecycleState::Active),
            "disabled" => Ok(CredentialLifecycleState::Disabled),
            "revoked" => Ok(CredentialLifecycleState::Revoked),
            _ => Err(DatabaseError::InvalidData),
        }
    }

    fn map_row(row: CredentialRow) -> Result<Credential, DatabaseError> {
        let kind = Self::map_kind(&row.kind)?;
        let state = Self::map_state(&row.lifecycle_state)?;

        Ok(Credential::restore(
            CredentialId::from_uuid(row.id),
            IdentityId::from_uuid(row.identity_id),
            kind,
            state,
        ))
    }
}

impl CredentialRepository for PostgresCredentialRepository {
    type Error = DatabaseError;

    fn find_by_id(&self, id: CredentialId) -> Result<Option<Credential>, Self::Error> {
        let row = self
            .block_on(async {
                sqlx::query_as::<_, CredentialRow>(
                    r#"
                    SELECT
                        id,
                        identity_id,
                        kind,
                        lifecycle_state
                    FROM credentials
                    WHERE id = $1
                    "#,
                )
                .bind(id.as_uuid())
                .fetch_optional(&self.pool)
                .await
            })
            .map_err(DatabaseError::Connection)?;

        row.map(Self::map_row).transpose()
    }

    fn find_by_identity_id(&self, identity_id: IdentityId) -> Result<Vec<Credential>, Self::Error> {
        let rows = self
            .block_on(async {
                sqlx::query_as::<_, CredentialRow>(
                    r#"
                    SELECT
                        id,
                        identity_id,
                        kind,
                        lifecycle_state
                    FROM credentials
                    WHERE identity_id = $1
                    "#,
                )
                .bind(identity_id.as_uuid())
                .fetch_all(&self.pool)
                .await
            })
            .map_err(DatabaseError::Connection)?;

        rows.into_iter().map(Self::map_row).collect()
    }

    fn save(&mut self, credential: Credential) -> Result<(), Self::Error> {
        let kind = match credential.kind() {
            CredentialKind::Password => "password",
            CredentialKind::Passkey => "passkey",
            CredentialKind::ApiKey => "api_key",
        };

        let lifecycle_state = match credential.lifecycle_state() {
            CredentialLifecycleState::Active => "active",
            CredentialLifecycleState::Disabled => "disabled",
            CredentialLifecycleState::Revoked => "revoked",
        };

        self.block_on(async {
            sqlx::query(
                r#"
                INSERT INTO credentials (
                    id,
                    identity_id,
                    kind,
                    lifecycle_state
                )
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (id)
                DO UPDATE SET
                    identity_id = EXCLUDED.identity_id,
                    kind = EXCLUDED.kind,
                    lifecycle_state = EXCLUDED.lifecycle_state
                "#,
            )
            .bind(credential.id().as_uuid())
            .bind(credential.identity_id().as_uuid())
            .bind(kind)
            .bind(lifecycle_state)
            .execute(&self.pool)
            .await
        })
        .map_err(DatabaseError::Connection)?;

        Ok(())
    }
}
