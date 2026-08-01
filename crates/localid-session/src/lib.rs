#![deny(missing_docs)]

//! Domain model for LocalID sessions.
//!
//! This crate contains the core Session domain model without dependencies on
//! transport protocols, persistence technologies, or application frameworks.

mod session;

pub use session::{Session, SessionError, SessionId, SessionLifecycleState};
