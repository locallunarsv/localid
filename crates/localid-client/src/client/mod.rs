//! Client aggregate.

mod aggregate;
mod error;
mod id;
mod lifecycle_state;

pub use aggregate::Client;
pub use error::ClientError;
pub use id::ClientId;
pub use lifecycle_state::ClientLifecycleState;
