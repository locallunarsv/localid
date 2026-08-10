//! OAuth response payloads.

mod authorize;
mod discovery;
mod token;
mod userinfo;

pub use authorize::AuthorizeResponseBody;
pub use discovery::DiscoveryResponseBody;
pub use token::TokenResponseBody;
pub use userinfo::UserInfoResponseBody;
