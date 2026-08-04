#![deny(missing_docs)]

//! HTTP API layer for LocalID.

/// Authentication utilities for protected API access.
pub mod auth;
mod error;
mod handler;
mod request;
mod response;
mod router;
mod state;

pub use error::ApiError;
pub use router::create_router;
pub use state::AppState;
/// Application bootstrap and dependency wiring.
pub mod bootstrap;
