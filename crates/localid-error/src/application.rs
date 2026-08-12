//! Application error mappings.

use localid_application::oauth::token_exchange::TokenExchangeError;

use crate::OAuthError;

impl From<TokenExchangeError> for OAuthError {
    fn from(error: TokenExchangeError) -> Self {
        match error {
            TokenExchangeError::AuthorizationCodeNotFound
            | TokenExchangeError::ClientMismatch
            | TokenExchangeError::RedirectUriMismatch
            | TokenExchangeError::InvalidCodeVerifier => Self::InvalidGrant,
            TokenExchangeError::CodeExpired | TokenExchangeError::CodeConsumed => {
                Self::InvalidGrant
            }

            TokenExchangeError::ClientNotFound => Self::InvalidClient,

            TokenExchangeError::AuthorizationCodeRepositoryFailure
            | TokenExchangeError::OAuthClientRepositoryFailure
            | TokenExchangeError::TokenIssuanceFailure => Self::ServerError,
            TokenExchangeError::IdTokenIssuanceFailure => Self::ServerError,
        }
    }
}

#[cfg(test)]
mod tests {
    use localid_application::oauth::token_exchange::TokenExchangeError;

    use crate::OAuthError;

    #[test]
    fn expired_authorization_code_maps_to_invalid_grant() {
        let error = TokenExchangeError::CodeExpired;

        let oauth_error = OAuthError::from(error);

        assert_eq!(oauth_error, OAuthError::InvalidGrant);
    }

    #[test]
    fn repository_failure_maps_to_server_error() {
        let error = TokenExchangeError::AuthorizationCodeRepositoryFailure;

        let oauth_error = OAuthError::from(error);

        assert_eq!(oauth_error, OAuthError::ServerError);
    }
}
