#![deny(missing_docs)]

//! Application layer for LocalID.
//!
//! Contains application use cases that orchestrate
//! domain services.

pub mod authentication;
pub mod authorization;
pub mod session;

pub use authentication::{
    AuthenticationPort, LoginCommand, LoginUseCase, RefreshTokenAdapter, RefreshTokenPort,
    RefreshTokenUseCase, TokenResponse, VerifyTokenQuery, VerifyTokenResponse, VerifyTokenUseCase,
};

pub use authorization::{
    AuthorizationApplicationError, AuthorizationContextResolver, IdentityRoleAdapter,
    IdentityRolePort,
};

pub use session::{
    GetCurrentSessionUseCase, LogoutSessionUseCase, SessionAdapter, SessionPort, SessionResponse,
};

mod error;

pub use error::ApplicationError;
