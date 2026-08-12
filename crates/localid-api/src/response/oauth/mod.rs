//! OAuth response payloads.

mod discovery;
mod jwks;
mod token;
mod userinfo;

pub mod redirect;

pub use discovery::DiscoveryResponseBody;
pub use jwks::JwksResponseBody;
pub use token::TokenResponseBody;
pub use userinfo::UserInfoResponseBody;
