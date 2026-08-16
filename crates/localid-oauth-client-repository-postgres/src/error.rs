/// PostgreSQL OAuth client repository error.
#[derive(Debug)]
pub enum PostgresOAuthClientRepositoryError {
    /// Database operation failed.
    Database(sqlx::Error),

    /// Stored data could not be reconstructed.
    InvalidData,
}

impl From<sqlx::Error> for PostgresOAuthClientRepositoryError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}
