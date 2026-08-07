use thiserror::Error;

/// Errors returned during OAuth token exchange.
#[derive(Debug, Error)]
pub enum TokenExchangeError {
    /// Authorization code was not found.
    #[error("authorization code not found")]
    AuthorizationCodeNotFound,

    /// OAuth client was not found.
    #[error("oauth client not found")]
    ClientNotFound,

    /// Authorization code belongs to another client.
    #[error("authorization code client mismatch")]
    ClientMismatch,

    /// Redirect URI does not match registered value.
    #[error("redirect uri mismatch")]
    RedirectUriMismatch,

    /// Authorization code is expired.
    #[error("authorization code expired")]
    CodeExpired,

    /// Authorization code has already been consumed.
    #[error("authorization code already consumed")]
    CodeConsumed,

    /// Authorization code repository failed.
    #[error("authorization code repository failure")]
    AuthorizationCodeRepositoryFailure,

    /// OAuth client repository failed.
    #[error("oauth client repository failure")]
    OAuthClientRepositoryFailure,

    /// Token issuance failed.
    #[error("token issuance failure")]
    TokenIssuanceFailure,
}
