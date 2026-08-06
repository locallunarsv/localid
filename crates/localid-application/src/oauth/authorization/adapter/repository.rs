use localid_oauth_authorization::{AuthorizationCode, AuthorizationCodeRepository};

use crate::oauth::authorization::AuthorizationPort;
use localid_oauth_client::{OAuthClient, OAuthClientRepository};

/// Adapter for OAuth authorization repositories.
pub struct AuthorizationRepositoryAdapter<C, A> {
    client_repository: C,
    code_repository: A,
}

impl<C, A> AuthorizationRepositoryAdapter<C, A> {
    /// Creates a new authorization repository adapter.
    #[must_use]
    pub const fn new(client_repository: C, code_repository: A) -> Self {
        Self {
            client_repository,
            code_repository,
        }
    }
}

impl<C, A> AuthorizationPort for AuthorizationRepositoryAdapter<C, A>
where
    C: OAuthClientRepository,
    A: AuthorizationCodeRepository,
{
    type Error = ();

    fn find_client(&self, client_id: &str) -> Result<Option<OAuthClient>, Self::Error> {
        self.client_repository
            .find_by_client_id(client_id)
            .map_err(|_| ())
    }

    fn save_code(&mut self, code: AuthorizationCode) -> Result<(), Self::Error> {
        self.code_repository.save(code).map_err(|_| ())
    }
}
