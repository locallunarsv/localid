use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::Handle;

use localid_config::DatabaseConfig;
use localid_credential::CredentialId;
use localid_password::{PasswordHash, PasswordMaterial};
use localid_repository::PasswordMaterialRepository;

use crate::DatabaseError;

#[derive(Debug, sqlx::FromRow)]
struct PasswordMaterialRow {
    credential_id: uuid::Uuid,
    password_hash: String,
}

/// PostgreSQL repository for PasswordMaterial.
#[derive(Clone)]
pub struct PostgresPasswordMaterialRepository {
    pool: PgPool,
    runtime: Handle,
}

impl PostgresPasswordMaterialRepository {
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

    fn map_row(row: PasswordMaterialRow) -> PasswordMaterial {
        PasswordMaterial::new(
            CredentialId::from_uuid(row.credential_id),
            PasswordHash::new(row.password_hash),
        )
    }
}

impl PasswordMaterialRepository for PostgresPasswordMaterialRepository {
    type Error = DatabaseError;

    fn find_by_credential_id(
        &self,
        credential_id: CredentialId,
    ) -> Result<Option<PasswordMaterial>, Self::Error> {
        let row = self
            .block_on(async {
                sqlx::query_as::<_, PasswordMaterialRow>(
                    r#"
                    SELECT
                        credential_id,
                        password_hash
                    FROM password_materials
                    WHERE credential_id = $1
                    "#,
                )
                .bind(credential_id.as_uuid())
                .fetch_optional(&self.pool)
                .await
            })
            .map_err(DatabaseError::Connection)?;

        Ok(row.map(Self::map_row))
    }

    fn save(&mut self, material: PasswordMaterial) -> Result<(), Self::Error> {
        self.block_on(async {
            sqlx::query(
                r#"
                INSERT INTO password_materials (
                    credential_id,
                    password_hash
                )
                VALUES ($1, $2)
                ON CONFLICT (credential_id)
                DO UPDATE SET
                    password_hash = EXCLUDED.password_hash
                "#,
            )
            .bind(material.credential_id().as_uuid())
            .bind(material.password_hash().as_str())
            .execute(&self.pool)
            .await
        })
        .map_err(DatabaseError::Connection)?;

        Ok(())
    }
}
