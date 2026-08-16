use localid_oauth_authorization::{AuthorizationCode, AuthorizationCodeRepository};
use localid_oauth_client::{OAuthClient, OAuthClientRepository};

use crate::oauth::authorization::AuthorizationPort;

/// Repository based authorization adapter.
pub struct AuthorizationRepositoryAdapter<C, A> {
    pub(crate) client_repository: C,
    pub(crate) code_repository: A,
}

impl<C, A> AuthorizationRepositoryAdapter<C, A> {
    /// Creates a new repository adapter.
    #[must_use]
    pub const fn new(client_repository: C, code_repository: A) -> Self {
        Self {
            client_repository,
            code_repository,
        }
    }
}

/// Repository adapter error.
#[derive(Debug)]
pub enum AuthorizationRepositoryError<CE> {
    /// OAuth client repository error.
    Client(CE),

    /// Authorization code repository error.
    Code(()),
}

impl<C, A> AuthorizationPort for AuthorizationRepositoryAdapter<C, A>
where
    C: OAuthClientRepository,
    A: AuthorizationCodeRepository<Error = ()>,
{
    type Error = AuthorizationRepositoryError<C::Error>;

    fn find_client(&self, client_id: &str) -> Result<Option<OAuthClient>, Self::Error> {
        self.client_repository
            .find_by_client_id(client_id)
            .map_err(AuthorizationRepositoryError::Client)
    }

    fn save_code(&mut self, code: AuthorizationCode) -> Result<(), Self::Error> {
        self.code_repository
            .save(code)
            .map_err(AuthorizationRepositoryError::Code)
    }
}
