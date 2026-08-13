use localid_crypto::hash_secret;

use super::{ClientAuthenticationCommand, ClientAuthenticationError, ClientAuthenticationPort};

/// OAuth client authentication use case.
pub struct ClientAuthenticationUseCase<P> {
    port: P,
}

impl<P> ClientAuthenticationUseCase<P> {
    /// Creates a new client authentication use case.
    #[must_use]
    pub const fn new(port: P) -> Self {
        Self { port }
    }
}

impl<P> ClientAuthenticationUseCase<P>
where
    P: ClientAuthenticationPort,
{
    /// Authenticates OAuth client credentials.
    pub fn execute(
        &self,
        command: ClientAuthenticationCommand,
    ) -> Result<localid_oauth_client::OAuthClient, ClientAuthenticationError> {
        let client = self
            .port
            .find_client(command.client_id())
            .map_err(|_| ClientAuthenticationError::RepositoryFailure)?
            .ok_or(ClientAuthenticationError::ClientNotFound)?;

        let secret_hash = hash_secret(command.client_secret());

        if secret_hash != client.secret_hash() {
            return Err(ClientAuthenticationError::InvalidSecret);
        }

        Ok(client)
    }
}
