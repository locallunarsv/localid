use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::Handle;
use uuid::Uuid;

use chrono::{DateTime, Utc};

use localid_client::ClientId;
use localid_config::DatabaseConfig;
use localid_identity::IdentityId;
use localid_repository::SessionRepository;
use localid_session::{Session, SessionId, SessionLifecycleState};

use crate::DatabaseError;

#[derive(Debug, sqlx::FromRow)]
struct SessionRow {
    id: Uuid,
    identity_id: Uuid,
    client_id: Uuid,
    lifecycle_state: String,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

/// PostgreSQL repository for Session aggregates.
#[derive(Clone)]
pub struct PostgresSessionRepository {
    pool: PgPool,
    runtime: Handle,
}

impl PostgresSessionRepository {
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

    fn map_state(value: &str) -> Result<SessionLifecycleState, DatabaseError> {
        match value {
            "active" => Ok(SessionLifecycleState::Active),
            "revoked" => Ok(SessionLifecycleState::Revoked),
            _ => Err(DatabaseError::InvalidData),
        }
    }

    fn map_row(row: SessionRow) -> Result<Session, DatabaseError> {
        let state = Self::map_state(&row.lifecycle_state)?;

        Ok(Session::restore(
            SessionId::from_uuid(row.id),
            IdentityId::from_uuid(row.identity_id),
            ClientId::from_uuid(row.client_id),
            state,
            row.created_at,
            row.expires_at,
        ))
    }
}

impl SessionRepository for PostgresSessionRepository {
    type Error = DatabaseError;

    fn find_by_id(&self, id: SessionId) -> Result<Option<Session>, Self::Error> {
        let row = self
            .block_on(async {
                sqlx::query_as::<_, SessionRow>(
                    r#"
                    SELECT
                        id,
                        identity_id,
                        client_id,
                        lifecycle_state,
                        created_at,
                        expires_at
                    FROM sessions
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

    fn find_by_identity_id(&self, identity_id: IdentityId) -> Result<Vec<Session>, Self::Error> {
        let rows = self
            .block_on(async {
                sqlx::query_as::<_, SessionRow>(
                    r#"
                    SELECT
                        id,
                        identity_id,
                        client_id,
                        lifecycle_state,
                        created_at,
                        expires_at
                    FROM sessions
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

    fn save(&mut self, session: Session) -> Result<(), Self::Error> {
        let state = match session.lifecycle_state() {
            SessionLifecycleState::Active => "active",
            SessionLifecycleState::Revoked => "revoked",
        };

        self.block_on(async {
            sqlx::query(
                r#"
                INSERT INTO sessions (
                    id,
                    identity_id,
                    client_id,
                    lifecycle_state,
                    created_at,
                    expires_at
                )
                VALUES ($1,$2,$3,$4,$5,$6)
                ON CONFLICT (id)
                DO UPDATE SET
                    lifecycle_state = EXCLUDED.lifecycle_state,
                    expires_at = EXCLUDED.expires_at
                "#,
            )
            .bind(session.id().as_uuid())
            .bind(session.identity_id().as_uuid())
            .bind(session.client_id().as_uuid())
            .bind(state)
            .bind(session.created_at())
            .bind(session.expires_at())
            .execute(&self.pool)
            .await
        })
        .map_err(DatabaseError::Connection)?;

        Ok(())
    }
}
