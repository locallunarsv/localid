use chrono::{DateTime, Utc};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::Handle;
use uuid::Uuid;

use localid_config::DatabaseConfig;
use localid_repository::TokenRepository;
use localid_session::SessionId;
use localid_token::{Token, TokenId, TokenLifecycleState};

use crate::DatabaseError;

#[derive(Debug, sqlx::FromRow)]
struct TokenRow {
    id: Uuid,
    session_id: Uuid,
    secret_hash: String,
    lifecycle_state: String,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

/// PostgreSQL repository for Token aggregates.
#[derive(Clone)]
pub struct PostgresTokenRepository {
    pool: PgPool,
    runtime: Handle,
}

impl PostgresTokenRepository {
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

    fn map_state(value: &str) -> Result<TokenLifecycleState, DatabaseError> {
        match value {
            "active" => Ok(TokenLifecycleState::Active),
            "revoked" => Ok(TokenLifecycleState::Revoked),

            _ => Err(DatabaseError::InvalidData),
        }
    }

    fn map_row(row: TokenRow) -> Result<Token, DatabaseError> {
        let state = Self::map_state(&row.lifecycle_state)?;

        Ok(Token::restore(
            TokenId::from_uuid(row.id),
            SessionId::from_uuid(row.session_id),
            row.secret_hash,
            state,
            row.created_at,
            row.expires_at,
        ))
    }
}

impl TokenRepository for PostgresTokenRepository {
    type Error = DatabaseError;

    fn find_by_id(&self, id: TokenId) -> Result<Option<Token>, Self::Error> {
        let row = self
            .block_on(async {
                sqlx::query_as::<_, TokenRow>(
                    r#"
                    SELECT
                        id,
                        session_id,
                        secret_hash,
                        lifecycle_state,
                        created_at,
                        expires_at
                    FROM tokens
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

    fn find_by_secret_hash(&self, secret_hash: &str) -> Result<Option<Token>, Self::Error> {
        let row = self
            .block_on(async {
                sqlx::query_as::<_, TokenRow>(
                    r#"
                    SELECT
                        id,
                        session_id,
                        secret_hash,
                        lifecycle_state,
                        created_at,
                        expires_at
                    FROM tokens
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

    fn save(&mut self, token: Token) -> Result<(), Self::Error> {
        let state = match token.lifecycle_state() {
            TokenLifecycleState::Active => "active",
            TokenLifecycleState::Revoked => "revoked",
        };

        self.block_on(async {
            sqlx::query(
                r#"
                INSERT INTO tokens (
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
