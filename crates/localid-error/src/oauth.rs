//! OAuth protocol errors.

use thiserror::Error;

/// Errors defined by OAuth protocol mapping.
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum OAuthError {
    /// Client request is malformed.
    #[error("invalid request")]
    InvalidRequest,

    /// Client authentication failed.
    #[error("invalid client")]
    InvalidClient,

    /// Authorization grant is invalid.
    #[error("invalid grant")]
    InvalidGrant,

    /// Requested scope is invalid.
    #[error("invalid scope")]
    InvalidScope,

    /// Unexpected server failure.
    #[error("server error")]
    ServerError,
}
