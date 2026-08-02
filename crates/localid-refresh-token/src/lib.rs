#![deny(missing_docs)]

//! Refresh Token domain for LocalID.

mod refresh_token;

pub use refresh_token::{
    RefreshToken, RefreshTokenError, RefreshTokenId, RefreshTokenLifecycleState,
};
