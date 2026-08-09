use std::sync::Arc;

use tokio::sync::Mutex;

use localid_application::oauth::token_exchange::TokenExchangeUseCase;

/// Shared state for the OAuth token exchange flow.
///
/// Holds the token exchange use case behind an asynchronous mutex so it can
/// be safely shared across Axum request handlers.
pub struct OAuthTokenState<REX, TEX> {
    /// Token exchange use case used by the OAuth token endpoint.
    pub token_exchange_use_case: Arc<Mutex<TokenExchangeUseCase<REX, TEX>>>,
}

impl<REX, TEX> OAuthTokenState<REX, TEX> {
    /// Creates a new OAuth token state.
    #[must_use]
    pub fn new(token_exchange_use_case: Arc<Mutex<TokenExchangeUseCase<REX, TEX>>>) -> Self {
        Self {
            token_exchange_use_case,
        }
    }
}
