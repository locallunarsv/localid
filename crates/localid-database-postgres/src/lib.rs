#![deny(missing_docs)]

//! PostgreSQL database infrastructure for LocalID.

mod error;
mod identity;
mod migration;
mod oauth_client;
mod pool;

pub use error::DatabaseError;
pub use identity::PostgresIdentityRepository;
pub use migration::migrate;
pub use oauth_client::PostgresOAuthClientRepository;
pub use pool::connect;
