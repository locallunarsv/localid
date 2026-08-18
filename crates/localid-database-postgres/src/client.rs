use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::Handle;
use uuid::Uuid;

use localid_client::{Client, ClientId, ClientLifecycleState};
use localid_config::DatabaseConfig;
use localid_repository::ClientRepository;

use crate::DatabaseError;

#[derive(Debug, sqlx::FromRow)]
struct ClientRow {
    id: Uuid,
    client_id: String,
    name: String,
    state: String,
}

/// PostgreSQL repository for Client aggregates.
#[derive(Clone)]
pub struct PostgresClientRepository {
    pool: PgPool,
    runtime: Handle,
}

impl PostgresClientRepository {
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

    fn map_state(value: &str) -> Result<ClientLifecycleState, DatabaseError> {
        match value {
            "active" => Ok(ClientLifecycleState::Active),
            "disabled" => Ok(ClientLifecycleState::Disabled),
            "deleted" => Ok(ClientLifecycleState::Deleted),

            _ => Err(DatabaseError::InvalidData),
        }
    }

    fn map_row(row: ClientRow) -> Result<Client, DatabaseError> {
        let state = Self::map_state(&row.state)?;

        Ok(Client::restore(
            ClientId::from_uuid(row.id),
            row.client_id,
            row.name,
            state,
        ))
    }

    fn block_on<F, T>(&self, future: F) -> T
    where
        F: std::future::Future<Output = T>,
    {
        tokio::task::block_in_place(|| self.runtime.block_on(future))
    }
}

impl ClientRepository for PostgresClientRepository {
    type Error = DatabaseError;

    fn find_by_id(&self, id: ClientId) -> Result<Option<Client>, Self::Error> {
        let row = self
            .block_on(async {
                sqlx::query_as::<_, ClientRow>(
                    r#"
                    SELECT
                        id,
                        client_id,
                        name,
                        state
                    FROM clients
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

    fn find_by_client_id(&self, client_id: &str) -> Result<Option<Client>, Self::Error> {
        let row = self
            .block_on(async {
                sqlx::query_as::<_, ClientRow>(
                    r#"
                    SELECT
                        id,
                        client_id,
                        name,
                        state
                    FROM clients
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

    fn save(&mut self, client: Client) -> Result<(), Self::Error> {
        let state = match client.state() {
            ClientLifecycleState::Active => "active",
            ClientLifecycleState::Disabled => "disabled",
            ClientLifecycleState::Deleted => "deleted",
        };

        self.block_on(async {
            sqlx::query(
                r#"
                INSERT INTO clients (
                    id,
                    client_id,
                    name,
                    state
                )
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (id)
                DO UPDATE SET
                    client_id = EXCLUDED.client_id,
                    name = EXCLUDED.name,
                    state = EXCLUDED.state
                "#,
            )
            .bind(client.id().as_uuid())
            .bind(client.client_id())
            .bind(client.name())
            .bind(state)
            .execute(&self.pool)
            .await
        })
        .map_err(DatabaseError::Connection)?;

        Ok(())
    }
}
