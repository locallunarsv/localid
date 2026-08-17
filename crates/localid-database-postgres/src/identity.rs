use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::Handle;
use uuid::Uuid;

use localid_config::DatabaseConfig;
use localid_identity::{Identity, IdentityId, LifecycleState};
use localid_repository::IdentityRepository;

use crate::DatabaseError;

#[derive(Debug, sqlx::FromRow)]
struct IdentityRow {
    id: Uuid,
    lifecycle_state: String,
}

/// PostgreSQL repository for Identity aggregates.
#[derive(Clone)]
pub struct PostgresIdentityRepository {
    pool: PgPool,
    runtime: Handle,
}

impl PostgresIdentityRepository {
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

    fn map_state(value: &str) -> Result<LifecycleState, DatabaseError> {
        match value {
            "active" => Ok(LifecycleState::Active),
            "disabled" => Ok(LifecycleState::Disabled),
            "deleted" => Ok(LifecycleState::Deleted),

            _ => Err(DatabaseError::InvalidData),
        }
    }

    fn map_row(row: IdentityRow) -> Result<Identity, DatabaseError> {
        let state = Self::map_state(&row.lifecycle_state)?;

        Ok(Identity::restore(IdentityId::from_uuid(row.id), state))
    }
}

impl IdentityRepository for PostgresIdentityRepository {
    type Error = DatabaseError;

    fn find_by_id(&self, id: IdentityId) -> Result<Option<Identity>, Self::Error> {
        let row = self
            .runtime
            .block_on(async {
                sqlx::query_as::<_, IdentityRow>(
                    r#"
                    SELECT
                        id,
                        lifecycle_state
                    FROM identities
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

    fn save(&mut self, identity: Identity) -> Result<(), Self::Error> {
        let lifecycle_state = match identity.lifecycle_state() {
            LifecycleState::Active => "active",
            LifecycleState::Disabled => "disabled",
            LifecycleState::Deleted => "deleted",
        };

        self.runtime
            .block_on(async {
                sqlx::query(
                    r#"
                    INSERT INTO identities (
                        id,
                        lifecycle_state
                    )
                    VALUES ($1, $2)
                    ON CONFLICT (id)
                    DO UPDATE SET
                        lifecycle_state = EXCLUDED.lifecycle_state
                    "#,
                )
                .bind(identity.id().as_uuid())
                .bind(lifecycle_state)
                .execute(&self.pool)
                .await
            })
            .map_err(DatabaseError::Connection)?;

        Ok(())
    }
}
