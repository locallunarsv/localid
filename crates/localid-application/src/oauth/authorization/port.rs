use localid_oauth_authorization::AuthorizationCode;
use localid_oauth_client::OAuthClient;

/// Port for OAuth authorization operations.
pub trait AuthorizationPort {
    /// Error returned by authorization operations.
    type Error;

    /// Finds a registered OAuth client by public client identifier.
    fn find_client(&self, client_id: &str) -> Result<Option<OAuthClient>, Self::Error>;

    /// Stores generated authorization code.
    fn save_code(&mut self, code: AuthorizationCode) -> Result<(), Self::Error>;
}
