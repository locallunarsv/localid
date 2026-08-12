use localid_oauth_authorization::{AuthorizationCode, AuthorizationCodeRepository};

use localid_oauth_client::{OAuthClient, OAuthClientRepository};

use crate::oauth::token_exchange::TokenExchangePort;

/// Repository adapter for OAuth token exchange flow.
pub struct TokenExchangeRepositoryAdapter<C, A> {
    client_repository: C,
    authorization_code_repository: A,
}

impl<C, A> TokenExchangeRepositoryAdapter<C, A> {
    /// Creates a new token exchange repository adapter.
    #[must_use]
    pub const fn new(client_repository: C, authorization_code_repository: A) -> Self {
        Self {
            client_repository,
            authorization_code_repository,
        }
    }
}

impl<C, A> TokenExchangePort for TokenExchangeRepositoryAdapter<C, A>
where
    C: OAuthClientRepository<Error = ()>,
    A: AuthorizationCodeRepository<Error = ()>,
{
    type Error = ();

    fn find_authorization_code_by_hash(
        &self,
        hash: &str,
    ) -> Result<Option<AuthorizationCode>, Self::Error> {
        self.authorization_code_repository.find_by_hash(hash)
    }

    fn save_authorization_code(&mut self, code: AuthorizationCode) -> Result<(), Self::Error> {
        self.authorization_code_repository.save(code)
    }
    fn find_client(&self, client_id: &str) -> Result<Option<OAuthClient>, Self::Error> {
        self.client_repository.find_by_client_id(client_id)
    }
}
