use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::Handle;

use localid_config::DatabaseConfig;
use localid_identity::IdentityId;
use localid_permission::Permission;
use localid_repository::IdentityRoleRepository;
use localid_role::Role;

use crate::DatabaseError;

/// PostgreSQL repository for resolving identity roles.
#[derive(Clone)]
pub struct PostgresIdentityRoleRepository {
    pool: PgPool,
    runtime: Handle,
}

#[derive(Debug, sqlx::FromRow)]
struct IdentityRoleRow {
    role_name: String,
    permission_name: Option<String>,
}

impl PostgresIdentityRoleRepository {
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
}

impl IdentityRoleRepository for PostgresIdentityRoleRepository {
    type Error = DatabaseError;

    fn find_roles(&self, identity_id: IdentityId) -> Result<Vec<Role>, Self::Error> {
        let rows = self
            .block_on(async {
                sqlx::query_as::<_, IdentityRoleRow>(
                    r#"
                    SELECT
                        r.name AS role_name,
                        p.name AS permission_name
                    FROM identity_roles ir
                    JOIN roles r
                        ON r.id = ir.role_id
                    LEFT JOIN role_permissions rp
                        ON rp.role_id = r.id
                    LEFT JOIN permissions p
                        ON p.id = rp.permission_id
                    WHERE ir.identity_id = $1
                    ORDER BY r.name
                    "#,
                )
                .bind(identity_id.as_uuid())
                .fetch_all(&self.pool)
                .await
            })
            .map_err(DatabaseError::Connection)?;

        let mut roles: Vec<Role> = Vec::new();

        for row in rows {
            let permission = row
                .permission_name
                .map(Permission::new)
                .transpose()
                .map_err(|_| DatabaseError::InvalidData)?;

            match roles.iter_mut().find(|role| role.name() == row.role_name) {
                Some(existing) => {
                    if let Some(permission) = permission {
                        let mut permissions = existing.permissions().to_vec();

                        if !permissions.contains(&permission) {
                            permissions.push(permission);
                        }

                        *existing = Role::new(existing.name().to_string(), permissions)
                            .map_err(|_| DatabaseError::InvalidData)?;
                    }
                }

                None => {
                    let permissions = permission.into_iter().collect();

                    let role = Role::new(row.role_name, permissions)
                        .map_err(|_| DatabaseError::InvalidData)?;

                    roles.push(role);
                }
            }
        }

        Ok(roles)
    }
}
