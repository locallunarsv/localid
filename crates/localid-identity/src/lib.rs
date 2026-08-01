#![deny(missing_docs)]

//! Domain model for LocalID identities.
//!
//! This crate contains the core Identity domain model without dependencies on
//! transport protocols, persistence technologies, or application frameworks.

mod identity;

pub use identity::{Identity, IdentityError, IdentityId, LifecycleState};
