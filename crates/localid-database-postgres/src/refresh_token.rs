use chrono::{DateTime, Utc};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::Handle;
use uuid::Uuid;

use localid_config::DatabaseConfig;
use localid_refresh_token::{RefreshToken, RefreshTokenId, RefreshTokenLifecycleState};
use localid_repository::RefreshTokenRepository;
use localid_session::SessionId;

use crate::DatabaseError;

#[derive(Debug, sqlx::FromRow)]
struct RefreshTokenRow {
    id: Uuid,
    session_id: Uuid,
    secret_hash: String,
    lifecycle_state: String,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

/// PostgreSQL repository for RefreshToken aggregates.
#[derive(Clone)]
pub struct PostgresRefreshTokenRepository {
    pool: PgPool,
    runtime: Handle,
}

impl PostgresRefreshTokenRepository {
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

    fn map_state(value: &str) -> Result<RefreshTokenLifecycleState, DatabaseError> {
        match value {
            "active" => Ok(RefreshTokenLifecycleState::Active),
            "revoked" => Ok(RefreshTokenLifecycleState::Revoked),

            _ => Err(DatabaseError::InvalidData),
        }
    }

    fn map_row(row: RefreshTokenRow) -> Result<RefreshToken, DatabaseError> {
        let state = Self::map_state(&row.lifecycle_state)?;

        Ok(RefreshToken::restore(
            RefreshTokenId::from_uuid(row.id),
            SessionId::from_uuid(row.session_id),
            row.secret_hash,
            state,
            row.created_at,
            row.expires_at,
        ))
    }
}

impl RefreshTokenRepository for PostgresRefreshTokenRepository {
    type Error = DatabaseError;

    fn find_by_id(&self, id: RefreshTokenId) -> Result<Option<RefreshToken>, Self::Error> {
        let row = self
            .block_on(async {
                sqlx::query_as::<_, RefreshTokenRow>(
                    r#"
                    SELECT
                        id,
                        session_id,
                        secret_hash,
                        lifecycle_state,
                        created_at,
                        expires_at
                    FROM refresh_tokens
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

    fn find_by_secret_hash(&self, secret_hash: &str) -> Result<Option<RefreshToken>, Self::Error> {
        let row = self
            .block_on(async {
                sqlx::query_as::<_, RefreshTokenRow>(
                    r#"
                    SELECT
                        id,
                        session_id,
                        secret_hash,
                        lifecycle_state,
                        created_at,
                        expires_at
                    FROM refresh_tokens
                    WHERE secret_hash = $1
                    "#,
                )
                .bind(secret_hash)
                .fetch_optional(&self.pool)
                .await
            })
            .map_err(DatabaseError::Connection)?;

        row.map(Self::map_row).transpose()
    }

    fn save(&mut self, token: RefreshToken) -> Result<(), Self::Error> {
        let state = match token.lifecycle_state() {
            RefreshTokenLifecycleState::Active => "active",
            RefreshTokenLifecycleState::Revoked => "revoked",
        };

        self.block_on(async {
            sqlx::query(
                r#"
                INSERT INTO refresh_tokens (
                    id,
                    session_id,
                    secret_hash,
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
            .bind(token.id().as_uuid())
            .bind(token.session_id().as_uuid())
            .bind(token.secret_hash())
            .bind(state)
            .bind(token.created_at())
            .bind(token.expires_at())
            .execute(&self.pool)
            .await
        })
        .map_err(DatabaseError::Connection)?;

        Ok(())
    }
}
