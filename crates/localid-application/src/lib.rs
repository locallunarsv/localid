#![deny(missing_docs)]

//! Application layer for LocalID.
//!
//! Contains application use cases that orchestrate
//! domain services.

pub mod authentication;

mod error;

pub use authentication::{
    AuthenticationPort, LoginCommand, LoginUseCase, RefreshTokenAdapter, RefreshTokenPort,
    RefreshTokenUseCase, TokenResponse, VerifyTokenQuery, VerifyTokenResponse, VerifyTokenUseCase,
};

pub use error::ApplicationError;
