#![deny(missing_docs)]

//! PostgreSQL database infrastructure for LocalID.

mod credential;
mod error;
mod identity;
mod migration;
mod oauth_client;
mod pool;
mod refresh_token;
mod session;
mod token;

pub use credential::PostgresCredentialRepository;
pub use error::DatabaseError;
pub use identity::PostgresIdentityRepository;
pub use migration::migrate;
pub use oauth_client::PostgresOAuthClientRepository;
pub use pool::connect;
pub use refresh_token::PostgresRefreshTokenRepository;
pub use session::PostgresSessionRepository;
pub use token::PostgresTokenRepository;
