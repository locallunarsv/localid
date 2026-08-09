#![deny(missing_docs)]

//! Application layer for LocalID.

mod error;

/// Authentication application use cases.
pub mod authentication;

/// Authorization application services.
pub mod authorization;

/// Client application services.
pub mod client;

/// OAuth application services.
pub mod oauth;

/// Session application services.
pub mod session;

/// Identity application services.
pub mod identity;

pub use error::ApplicationError;

/// Authentication exports.
/// Authentication application use cases.
pub use authentication::{
    AuthenticationPort, LoginCommand, LoginUseCase, PasswordAuthenticationAdapter,
    RefreshTokenAdapter, RefreshTokenPort, RefreshTokenUseCase, TokenResponse,
    TokenVerificationAdapter, VerifyTokenQuery, VerifyTokenResponse, VerifyTokenUseCase,
};

/// Authorization exports.
pub use authorization::{AuthorizationContextResolver, IdentityRoleAdapter, IdentityRolePort};

/// Client exports.
pub use client::{ClientPort, ClientRepositoryAdapter, FindClientQuery, GetClientUseCase};

/// OAuth authorization exports.
/// OAuth authorization exports.
pub use oauth::authorization::{
    AuthorizationPort, AuthorizationRepositoryAdapter, AuthorizationResult, AuthorizeCommand,
    AuthorizeUseCase,
};

/// OAuth token exchange exports.
pub use oauth::token_exchange::{
    TokenExchangeCommand, TokenExchangeError, TokenExchangePort, TokenExchangeRepositoryAdapter,
    TokenExchangeResult, TokenExchangeUseCase,
};

/// Session exports.
/// Session exports.
pub use session::{
    GetCurrentSessionUseCase, LogoutSessionUseCase, SessionAdapter, SessionPort, SessionResponse,
};

/// Identity exports.
/// Identity exports.
pub use identity::{
    GetIdentityUseCase, IdentityLookupPort, IdentityLookupService, IdentityRepositoryAdapter,
    IdentityResult,
};
