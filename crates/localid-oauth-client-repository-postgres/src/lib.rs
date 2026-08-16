#![deny(missing_docs)]

//! PostgreSQL repository implementation for OAuth clients.

mod error;
mod migration;
mod repository;

pub use error::PostgresOAuthClientRepositoryError;
pub use migration::migrate;
pub use repository::PostgresOAuthClientRepository;
