#![deny(missing_docs)]

//! Domain model for LocalID credentials.
//!
//! This crate contains the core Credential domain model without dependencies on
//! transport protocols, persistence technologies, or application frameworks.

mod credential;

pub use credential::{
    Credential, CredentialError, CredentialId, CredentialKind, CredentialLifecycleState,
};
