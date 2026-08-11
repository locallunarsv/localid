use localid_authentication::TokenIssuanceService;

use super::{
    IdTokenIssuer, TokenExchangeCommand, TokenExchangeError, TokenExchangePort, TokenExchangeResult,
};

use localid_token::IdTokenClaims;

/// OAuth authorization code exchange use case.
#[derive(Debug)]
pub struct TokenExchangeUseCase<R, T, I> {
    repository: R,
    token_issuer: T,
    id_token_issuer: I,
}

impl<R, T, I> TokenExchangeUseCase<R, T, I> {
    /// Creates a new token exchange use case.
    #[must_use]
    pub const fn new(repository: R, token_issuer: T, id_token_issuer: I) -> Self {
        Self {
            repository,
            token_issuer,
            id_token_issuer,
        }
    }
}

impl<R, T, I> TokenExchangeUseCase<R, T, I>
where
    R: TokenExchangePort + Send + Sync,
    T: TokenIssuanceService + Send + Sync,
    I: IdTokenIssuer + Send + Sync,
{
    /// Executes OAuth authorization code exchange.
    pub fn execute(
        &mut self,
        command: TokenExchangeCommand,
    ) -> Result<TokenExchangeResult, TokenExchangeError> {
        let mut authorization_code = self
            .repository
            .find_authorization_code(command.code_id())
            .map_err(|_| TokenExchangeError::AuthorizationCodeRepositoryFailure)?
            .ok_or(TokenExchangeError::AuthorizationCodeNotFound)?;

        let client = self
            .repository
            .find_client(command.client_id())
            .map_err(|_| TokenExchangeError::OAuthClientRepositoryFailure)?
            .ok_or(TokenExchangeError::ClientNotFound)?;

        if authorization_code.client_id() != client.id() {
            return Err(TokenExchangeError::ClientMismatch);
        }

        if authorization_code.redirect_uri() != command.redirect_uri() {
            return Err(TokenExchangeError::RedirectUriMismatch);
        }

        if !authorization_code.is_active_at(chrono::Utc::now()) {
            if authorization_code.is_expired_at(chrono::Utc::now()) {
                return Err(TokenExchangeError::CodeExpired);
            }

            return Err(TokenExchangeError::CodeConsumed);
        }

        let identity_id = authorization_code.identity_id();

        authorization_code
            .consume()
            .map_err(|_| TokenExchangeError::CodeConsumed)?;

        self.repository
            .save_authorization_code(authorization_code)
            .map_err(|_| TokenExchangeError::AuthorizationCodeRepositoryFailure)?;

        let authentication_result = self
            .token_issuer
            .issue(identity_id, client.local_client_id())
            .map_err(|_| TokenExchangeError::TokenIssuanceFailure)?;

        let now = chrono::Utc::now().timestamp();

        let claims = IdTokenClaims {
            iss: "http://localhost:8080".to_string(),
            sub: identity_id.to_string(),
            aud: command.client_id().to_string(),
            iat: now,
            exp: now + 3600,
            nonce: None,
        };

        let id_token = self
            .id_token_issuer
            .issue(claims)
            .map_err(|_| TokenExchangeError::IdTokenIssuanceFailure)?;

        Ok(TokenExchangeResult::new(
            authentication_result.token().secret(),
            authentication_result.refresh_token().secret(),
            id_token,
            authentication_result.session().expires_at(),
        ))
    }
}
