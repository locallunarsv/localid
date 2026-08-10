//! OAuth response payloads.

mod authorize;
mod discovery;
mod jwks;
mod token;
mod userinfo;

pub use authorize::AuthorizeResponseBody;
pub use discovery::DiscoveryResponseBody;
pub use jwks::JwksResponseBody;
pub use token::TokenResponseBody;
pub use userinfo::UserInfoResponseBody;
