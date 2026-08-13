use super::super::OAuthClient;
use super::super::OAuthClientId;

/// Repository port for OAuth clients.
pub trait OAuthClientRepository {
    /// Repository error.
    type Error;

    /// Saves OAuth client.
    fn save(&mut self, client: OAuthClient) -> Result<(), Self::Error>;

    /// Finds OAuth client by internal identifier.
    fn find_by_id(&self, id: OAuthClientId) -> Result<Option<OAuthClient>, Self::Error>;

    /// Finds OAuth client by public client identifier.
    fn find_by_client_id(&self, client_id: &str) -> Result<Option<OAuthClient>, Self::Error>;

    /// Finds all OAuth clients.
    fn find_all(&self) -> Result<Vec<OAuthClient>, Self::Error>;
}
