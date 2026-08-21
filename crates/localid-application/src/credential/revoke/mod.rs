//! Credential revocation application services.

mod command;
mod error;
mod revoke;

pub use command::RevokeCredentialCommand;
pub use error::RevokeCredentialError;
pub use revoke::RevokeCredentialUseCase;
