#![deny(missing_docs)]

//! Authentication orchestration for LocalID.
//!
//! This crate coordinates credential-specific authentication flows without
//! depending on transport protocols or concrete storage implementations.

mod authentication;

pub use authentication::{
    AuthenticatePasswordRequest, AuthenticateResult, AuthenticatedContext, AuthenticationError,
    DefaultPasswordAuthenticationService, DefaultRefreshTokenService, DefaultSessionFactory,
    DefaultSessionService, DefaultTokenValidator, DefaultTokenVerificationService,
    PasswordAuthenticationDependencies, PasswordAuthenticationService, RefreshResult,
    RefreshTokenService, SessionFactory, SessionService, TokenValidator, TokenVerificationResult,
    TokenVerificationService,
};
