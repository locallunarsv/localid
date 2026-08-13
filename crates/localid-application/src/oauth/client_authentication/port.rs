use localid_oauth_client::OAuthClient;

/// Port required for OAuth client authentication.
pub trait ClientAuthenticationPort {
    /// Repository error.
    type Error;

    /// Finds OAuth client by public client identifier.
    fn find_client(&self, client_id: &str) -> Result<Option<OAuthClient>, Self::Error>;
}
