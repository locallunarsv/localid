use localid_oauth_authorization::{AuthorizationCode, AuthorizationCodeId};
use localid_oauth_client::OAuthClient;

/// Port required by OAuth token exchange flow.
pub trait TokenExchangePort {
    /// Error returned by token exchange dependencies.
    type Error;

    /// Finds authorization code by identifier.
    fn find_authorization_code(
        &self,
        id: AuthorizationCodeId,
    ) -> Result<Option<AuthorizationCode>, Self::Error>;

    /// Saves authorization code changes.
    fn save_authorization_code(&mut self, code: AuthorizationCode) -> Result<(), Self::Error>;

    /// Finds OAuth client by public client id.
    fn find_client(&self, client_id: &str) -> Result<Option<OAuthClient>, Self::Error>;
}
