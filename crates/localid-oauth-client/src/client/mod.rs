//! OAuth client aggregate.

mod aggregate;
mod error;
mod id;
mod lifecycle_state;

pub use aggregate::OAuthClient;
pub use error::OAuthClientError;
pub use id::OAuthClientId;
pub use lifecycle_state::OAuthClientLifecycleState;
