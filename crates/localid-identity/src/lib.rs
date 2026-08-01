#![deny(missing_docs)]

//! Domain model for LocalID identities.
//!
//! This crate contains the core Identity domain model without dependencies on
//! transport protocols, persistence technologies, or application frameworks.

mod identity;
mod identity_error;
mod identity_id;
mod lifecycle_state;

pub use identity::Identity;
pub use identity_error::IdentityError;
pub use identity_id::IdentityId;
pub use lifecycle_state::LifecycleState;
