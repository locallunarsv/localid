#![deny(missing_docs)]

//! Application layer for LocalID.
//!
//! Contains application use cases that orchestrate
//! domain services.

pub mod authentication;

mod error;

pub use authentication::{
    AuthenticationPort, LoginCommand, LoginUseCase, PasswordAuthenticationAdapter,
    RefreshTokenAdapter, RefreshTokenPort, RefreshTokenUseCase, TokenResponse,
};

pub use error::ApplicationError;
