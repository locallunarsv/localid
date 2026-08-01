#![deny(missing_docs)]

//! In-memory storage adapters for LocalID repository contracts.
//!
//! This crate is intended for testing, development, and ephemeral LocalID
//! deployments. Cloned storage handles share the same in-memory state.

mod error;
mod storage;

pub use error::MemoryStorageError;
pub use storage::MemoryStorage;
