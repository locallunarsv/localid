use localid_client::ClientId;
use localid_crypto::hash_secret;
use localid_oauth_client::{OAuthClient, OAuthClientId, OAuthClientRepository};

use super::{CreateOAuthClientCommand, CreateOAuthClientError, CreateOAuthClientResult};

use rand::{Rng, distributions::Alphanumeric, thread_rng};

/// Creates OAuth clients.
pub struct CreateOAuthClientUseCase<R> {
    repository: R,
}

impl<R> CreateOAuthClientUseCase<R> {
    /// Creates use case.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CreateOAuthClientUseCase<R>
where
    R: OAuthClientRepository,
{
    /// Executes OAuth client creation.
    pub fn execute(
        &mut self,
        command: CreateOAuthClientCommand,
    ) -> Result<CreateOAuthClientResult, CreateOAuthClientError> {
        let client_id = format!("client-{}", OAuthClientId::new());

        let client_secret: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        let secret_hash = hash_secret(&client_secret);

        let local_client_id = ClientId::new();

        let client = OAuthClient::new(
            OAuthClientId::new(),
            local_client_id,
            client_id.clone(),
            command.name(),
            secret_hash,
            command.redirect_uris().to_vec(),
        );

        self.repository
            .save(client)
            .map_err(|_| CreateOAuthClientError::RepositoryFailure)?;

        Ok(CreateOAuthClientResult::new(client_id, client_secret))
    }
}
