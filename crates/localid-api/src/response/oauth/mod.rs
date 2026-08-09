//! OAuth response payloads.

mod authorize;
mod token;
mod userinfo;

pub use authorize::AuthorizeResponseBody;
pub use token::TokenResponseBody;

pub use userinfo::UserInfoResponseBody;
