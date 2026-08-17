/// Database errors.
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    /// PostgreSQL connection failure.
    #[error("database connection failure: {0}")]
    Connection(#[from] sqlx::Error),

    /// Invalid database data.
    #[error("invalid database data")]
    InvalidData,
}
