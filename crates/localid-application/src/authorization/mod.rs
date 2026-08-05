//! Authorization application capabilities.

mod adapter;
mod error;
mod port;
mod resolver;

pub use adapter::IdentityRoleAdapter;
pub use error::AuthorizationApplicationError;
pub use port::IdentityRolePort;
pub use resolver::AuthorizationContextResolver;
