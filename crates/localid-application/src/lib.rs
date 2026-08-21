#![deny(missing_docs)]

//! Application layer for LocalID.

mod error;

/// Authentication application use cases.
pub mod authentication;

/// Authorization application services.
pub mod authorization;

/// Client application services.
pub mod client;

/// Credential application services.
pub mod credential;

/// Identity application services.
pub mod identity;

/// OAuth application services.
pub mod oauth;

/// Session application services.
pub mod session;

pub use error::ApplicationError;

/// Authentication exports.
pub use authentication::{
    AuthenticationPort, LoginCommand, LoginUseCase, PasswordAuthenticationAdapter,
    RefreshTokenAdapter, RefreshTokenPort, RefreshTokenUseCase, TokenResponse,
    TokenVerificationAdapter, VerifyTokenQuery, VerifyTokenResponse, VerifyTokenUseCase,
};

/// Authorization exports.
pub use authorization::{AuthorizationContextResolver, IdentityRoleAdapter, IdentityRolePort};

/// Client exports.
pub use client::{ClientPort, ClientRepositoryAdapter, FindClientQuery, GetClientUseCase};

/// Password credential creation exports.
pub use credential::password::create::{
    CreatePasswordCredentialCommand, CreatePasswordCredentialError, CreatePasswordCredentialResult,
    CreatePasswordCredentialUseCase,
};

/// Identity exports.
pub use identity::{
    DeleteIdentityCommand, DeleteIdentityError, DeleteIdentityUseCase, DisableIdentityCommand,
    DisableIdentityError, DisableIdentityUseCase, EnableIdentityCommand, EnableIdentityError,
    EnableIdentityUseCase, GetIdentityUseCase, IdentityLookupPort, IdentityLookupService,
    IdentityRepositoryAdapter, IdentityResult, ListIdentitiesResult, ListIdentitiesUseCase,
};

/// OAuth authorization exports.
pub use oauth::authorization::{
    AuthorizationPort, AuthorizationRepositoryAdapter, AuthorizationResult, AuthorizeCommand,
    AuthorizeUseCase,
};

/// OAuth client authentication exports.
pub use oauth::client_authentication::{
    ClientAuthenticationCommand, ClientAuthenticationError, ClientAuthenticationPort,
    ClientAuthenticationUseCase,
};

/// OAuth token exchange exports.
pub use oauth::token_exchange::{
    TokenExchangeCommand, TokenExchangeError, TokenExchangePort, TokenExchangeRepositoryAdapter,
    TokenExchangeResult, TokenExchangeUseCase,
};

/// Session exports.
pub use session::{
    GetCurrentSessionUseCase, LogoutSessionUseCase, SessionAdapter, SessionPort, SessionResponse,
};

/// OAuth client creation exports.
pub use oauth::client::{
    CreateOAuthClientCommand, CreateOAuthClientError, CreateOAuthClientResult,
    CreateOAuthClientUseCase,
};

/// OAuth client lookup exports.
pub use oauth::client::{
    GetOAuthClientError, GetOAuthClientQuery, GetOAuthClientResult, GetOAuthClientUseCase,
};

/// OAuth client disable exports.
pub use oauth::client::{
    DisableOAuthClientCommand, DisableOAuthClientError, DisableOAuthClientUseCase,
};

/// OAuth client listing exports.
pub use oauth::client::{ListOAuthClientsError, ListOAuthClientsResult, ListOAuthClientsUseCase};

/// OAuth client activation exports.
pub use oauth::client::{
    ActivateOAuthClientCommand, ActivateOAuthClientError, ActivateOAuthClientUseCase,
};

/// OAuth client deletion exports.
pub use oauth::client::{
    DeleteOAuthClientCommand, DeleteOAuthClientError, DeleteOAuthClientUseCase,
};
