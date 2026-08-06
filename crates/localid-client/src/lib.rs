#![deny(missing_docs)]

//! Client domain for LocalID.
//!
//! Represents applications that use LocalID authentication services.

mod client;

pub use client::{Client, ClientError, ClientId, ClientLifecycleState};
