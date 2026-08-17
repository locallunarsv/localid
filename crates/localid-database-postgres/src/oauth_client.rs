use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::Handle;
use uuid::Uuid;

use localid_client::ClientId;
use localid_config::DatabaseConfig;
use localid_oauth_client::{
    OAuthClient, OAuthClientId, OAuthClientLifecycleState, OAuthClientRepository,
};

use crate::DatabaseError;

#[derive(Debug, sqlx::FromRow)]
struct OAuthClientRow {
    id: Uuid,
    local_client_id: Uuid,
    client_id: String,
    name: String,
    secret_hash: String,
    redirect_uris: serde_json::Value,
    state: String,
}

/// PostgreSQL OAuth client repository.
#[derive(Clone)]
pub struct PostgresOAuthClientRepository {
    pool: PgPool,
    runtime: Handle,
}

impl PostgresOAuthClientRepository {
    /// Creates repository from PostgreSQL pool.
    #[must_use]
    pub fn new(pool: PgPool, runtime: Handle) -> Self {
        Self { pool, runtime }
    }

    /// Returns PostgreSQL connection pool.
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

    fn map_state(value: &str) -> Result<OAuthClientLifecycleState, DatabaseError> {
        match value {
            "active" => Ok(OAuthClientLifecycleState::Active),
            "disabled" => Ok(OAuthClientLifecycleState::Disabled),
            "deleted" => Ok(OAuthClientLifecycleState::Deleted),

            _ => Err(DatabaseError::InvalidData),
        }
    }

    fn map_row(row: OAuthClientRow) -> Result<OAuthClient, DatabaseError> {
        let state = Self::map_state(&row.state)?;

        let redirect_uris: Vec<String> =
            serde_json::from_value(row.redirect_uris).map_err(|_| DatabaseError::InvalidData)?;

        Ok(OAuthClient::restore(
            OAuthClientId::from_uuid(row.id),
            ClientId::from_uuid(row.local_client_id),
            row.client_id,
            row.name,
            row.secret_hash,
            redirect_uris,
            state,
        ))
    }

    /// Clears OAuth clients.
    pub fn clear(&self) -> Result<(), DatabaseError> {
        self.block_on(async {
            sqlx::query(
                r#"
                TRUNCATE TABLE oauth_clients
                "#,
            )
            .execute(&self.pool)
            .await
        })
        .map(|_| ())
        .map_err(DatabaseError::Connection)
    }
}

impl OAuthClientRepository for PostgresOAuthClientRepository {
    type Error = DatabaseError;

    fn find_by_id(&self, id: OAuthClientId) -> Result<Option<OAuthClient>, Self::Error> {
        let row = self
            .block_on(async {
                sqlx::query_as::<_, OAuthClientRow>(
                    r#"
                    SELECT
                        id,
                        local_client_id,
                        client_id,
                        name,
                        secret_hash,
                        redirect_uris,
                        state
                    FROM oauth_clients
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

    fn find_by_client_id(&self, client_id: &str) -> Result<Option<OAuthClient>, Self::Error> {
        let row = self
            .block_on(async {
                sqlx::query_as::<_, OAuthClientRow>(
                    r#"
                    SELECT
                        id,
                        local_client_id,
                        client_id,
                        name,
                        secret_hash,
                        redirect_uris,
                        state
                    FROM oauth_clients
                    WHERE client_id = $1
                    "#,
                )
                .bind(client_id)
                .fetch_optional(&self.pool)
                .await
            })
            .map_err(DatabaseError::Connection)?;

        row.map(Self::map_row).transpose()
    }

    fn find_all(&self) -> Result<Vec<OAuthClient>, Self::Error> {
        let rows = self
            .block_on(async {
                sqlx::query_as::<_, OAuthClientRow>(
                    r#"
                    SELECT
                        id,
                        local_client_id,
                        client_id,
                        name,
                        secret_hash,
                        redirect_uris,
                        state
                    FROM oauth_clients
                    "#,
                )
                .fetch_all(&self.pool)
                .await
            })
            .map_err(DatabaseError::Connection)?;

        rows.into_iter().map(Self::map_row).collect()
    }

    fn save(&mut self, client: OAuthClient) -> Result<(), Self::Error> {
        let redirect_uris =
            serde_json::to_value(client.redirect_uris()).map_err(|_| DatabaseError::InvalidData)?;

        let state = match client.state() {
            OAuthClientLifecycleState::Active => "active",
            OAuthClientLifecycleState::Disabled => "disabled",
            OAuthClientLifecycleState::Deleted => "deleted",
        };

        self.block_on(async {
            sqlx::query(
                r#"
                INSERT INTO oauth_clients (
                    id,
                    local_client_id,
                    client_id,
                    name,
                    secret_hash,
                    redirect_uris,
                    state
                )
                VALUES ($1,$2,$3,$4,$5,$6,$7)
                ON CONFLICT (id)
                DO UPDATE SET
                    local_client_id = EXCLUDED.local_client_id,
                    client_id = EXCLUDED.client_id,
                    name = EXCLUDED.name,
                    secret_hash = EXCLUDED.secret_hash,
                    redirect_uris = EXCLUDED.redirect_uris,
                    state = EXCLUDED.state
                "#,
            )
            .bind(client.id().as_uuid())
            .bind(client.local_client_id().as_uuid())
            .bind(client.client_id())
            .bind(client.name())
            .bind(client.secret_hash())
            .bind(redirect_uris)
            .bind(state)
            .execute(&self.pool)
            .await
        })
        .map_err(DatabaseError::Connection)?;

        Ok(())
    }
}
