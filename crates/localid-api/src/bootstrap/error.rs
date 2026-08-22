//! Application bootstrap errors.

use std::{error::Error, fmt};

use localid_crypto::CryptoError;
use localid_database_postgres::DatabaseError;

/// Error returned while initializing LocalID application dependencies.
#[derive(Debug)]
pub enum BootstrapError {
    /// PostgreSQL initialization failed.
    Database(DatabaseError),

    /// Cryptographic key initialization failed.
    Crypto(CryptoError),
}

impl fmt::Display for BootstrapError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(error) => {
                write!(formatter, "database bootstrap failed: {error}")
            }
            Self::Crypto(error) => {
                write!(formatter, "cryptographic bootstrap failed: {error:?}")
            }
        }
    }
}

impl Error for BootstrapError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Database(error) => Some(error),
            Self::Crypto(_) => None,
        }
    }
}

impl From<DatabaseError> for BootstrapError {
    fn from(error: DatabaseError) -> Self {
        Self::Database(error)
    }
}

impl From<CryptoError> for BootstrapError {
    fn from(error: CryptoError) -> Self {
        Self::Crypto(error)
    }
}
