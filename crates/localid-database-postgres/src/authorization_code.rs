use chrono::{DateTime, Utc};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::runtime::Handle;
use uuid::Uuid;

use localid_config::DatabaseConfig;
use localid_identity::IdentityId;
use localid_oauth_authorization::{
    AuthorizationCode, AuthorizationCodeId, AuthorizationCodeLifecycleState,
    AuthorizationCodeRepository, CodeChallengeMethod,
};
use localid_oauth_client::OAuthClientId;

use crate::DatabaseError;

#[derive(Debug, sqlx::FromRow)]
struct AuthorizationCodeRow {
    id: Uuid,
    oauth_client_id: Uuid,
    identity_id: Uuid,

    code_hash: String,
    redirect_uri: String,

    nonce: Option<String>,
    scope: serde_json::Value,
    request_state: Option<String>,

    pkce_challenge: Option<String>,
    pkce_method: Option<String>,

    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,

    state: String,
}

/// PostgreSQL repository for AuthorizationCode aggregates.
#[derive(Clone)]
pub struct PostgresAuthorizationCodeRepository {
    pool: PgPool,
    runtime: Handle,
}

impl PostgresAuthorizationCodeRepository {
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

    fn map_state(value: &str) -> Result<AuthorizationCodeLifecycleState, DatabaseError> {
        match value {
            "active" => Ok(AuthorizationCodeLifecycleState::Active),
            "consumed" => Ok(AuthorizationCodeLifecycleState::Consumed),
            _ => Err(DatabaseError::InvalidData),
        }
    }

    fn map_pkce_method(value: Option<&str>) -> Result<Option<CodeChallengeMethod>, DatabaseError> {
        value
            .map(|value| CodeChallengeMethod::from_str(value).ok_or(DatabaseError::InvalidData))
            .transpose()
    }

    fn map_row(row: AuthorizationCodeRow) -> Result<AuthorizationCode, DatabaseError> {
        let state = Self::map_state(&row.state)?;

        let scope: Vec<String> =
            serde_json::from_value(row.scope).map_err(|_| DatabaseError::InvalidData)?;

        let pkce_method = Self::map_pkce_method(row.pkce_method.as_deref())?;

        Ok(AuthorizationCode::restore(
            AuthorizationCodeId::from_uuid(row.id),
            OAuthClientId::from_uuid(row.oauth_client_id),
            IdentityId::from_uuid(row.identity_id),
            row.code_hash,
            row.redirect_uri,
            row.nonce,
            scope,
            row.request_state,
            row.pkce_challenge,
            pkce_method,
            row.created_at,
            row.expires_at,
            state,
        ))
    }
}

impl AuthorizationCodeRepository for PostgresAuthorizationCodeRepository {
    type Error = DatabaseError;

    fn save(&mut self, code: AuthorizationCode) -> Result<(), Self::Error> {
        let scope = serde_json::to_value(code.scope()).map_err(|_| DatabaseError::InvalidData)?;

        let state = match code.state() {
            AuthorizationCodeLifecycleState::Active => "active",
            AuthorizationCodeLifecycleState::Consumed => "consumed",
            AuthorizationCodeLifecycleState::Expired => {
                return Err(DatabaseError::InvalidData);
            }
        };

        let pkce_method = code.pkce_method().map(CodeChallengeMethod::as_str);

        self.block_on(async {
            sqlx::query(
                r#"
                INSERT INTO authorization_codes (
                    id,
                    oauth_client_id,
                    identity_id,
                    code_hash,
                    redirect_uri,
                    nonce,
                    scope,
                    request_state,
                    pkce_challenge,
                    pkce_method,
                    created_at,
                    expires_at,
                    state
                )
                VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)
                ON CONFLICT (id)
                DO UPDATE SET
                    oauth_client_id = EXCLUDED.oauth_client_id,
                    identity_id = EXCLUDED.identity_id,
                    code_hash = EXCLUDED.code_hash,
                    redirect_uri = EXCLUDED.redirect_uri,
                    nonce = EXCLUDED.nonce,
                    scope = EXCLUDED.scope,
                    request_state = EXCLUDED.request_state,
                    pkce_challenge = EXCLUDED.pkce_challenge,
                    pkce_method = EXCLUDED.pkce_method,
                    expires_at = EXCLUDED.expires_at,
                    state = EXCLUDED.state
                "#,
            )
            .bind(code.id().as_uuid())
            .bind(code.client_id().as_uuid())
            .bind(code.identity_id().as_uuid())
            .bind(code.code_hash())
            .bind(code.redirect_uri())
            .bind(code.nonce())
            .bind(scope)
            .bind(code.request_state())
            .bind(code.pkce_challenge())
            .bind(pkce_method)
            .bind(code.created_at())
            .bind(code.expires_at())
            .bind(state)
            .execute(&self.pool)
            .await
        })
        .map_err(DatabaseError::Connection)?;

        Ok(())
    }

    fn find_by_id(
        &self,
        id: AuthorizationCodeId,
    ) -> Result<Option<AuthorizationCode>, Self::Error> {
        let row = self
            .block_on(async {
                sqlx::query_as::<_, AuthorizationCodeRow>(
                    r#"
                    SELECT
                        id,
                        oauth_client_id,
                        identity_id,
                        code_hash,
                        redirect_uri,
                        nonce,
                        scope,
                        request_state,
                        pkce_challenge,
                        pkce_method,
                        created_at,
                        expires_at,
                        state
                    FROM authorization_codes
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

    fn find_by_hash(&self, hash: &str) -> Result<Option<AuthorizationCode>, Self::Error> {
        let row = self
            .block_on(async {
                sqlx::query_as::<_, AuthorizationCodeRow>(
                    r#"
                    SELECT
                        id,
                        oauth_client_id,
                        identity_id,
                        code_hash,
                        redirect_uri,
                        nonce,
                        scope,
                        request_state,
                        pkce_challenge,
                        pkce_method,
                        created_at,
                        expires_at,
                        state
                    FROM authorization_codes
                    WHERE code_hash = $1
                    "#,
                )
                .bind(hash)
                .fetch_optional(&self.pool)
                .await
            })
            .map_err(DatabaseError::Connection)?;

        row.map(Self::map_row).transpose()
    }
}
