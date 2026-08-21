#![deny(missing_docs)]

//! PostgreSQL database infrastructure for LocalID.

mod authorization_code;
mod client;
mod credential;
mod error;
mod identity;
mod identity_role;
mod migration;
mod oauth_client;
mod password_material;
mod pool;
mod refresh_token;
mod session;
mod token;

pub use authorization_code::PostgresAuthorizationCodeRepository;
pub use credential::PostgresCredentialRepository;
pub use error::DatabaseError;
pub use identity::PostgresIdentityRepository;
pub use migration::migrate;
pub use oauth_client::PostgresOAuthClientRepository;
pub use pool::connect;
pub use refresh_token::PostgresRefreshTokenRepository;
pub use session::PostgresSessionRepository;
pub use token::PostgresTokenRepository;

pub use password_material::PostgresPasswordMaterialRepository;

pub use identity_role::PostgresIdentityRoleRepository;

pub use client::PostgresClientRepository;
