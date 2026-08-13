use localid_application::ClientAuthenticationPort;

use localid_oauth_client::{OAuthClient, OAuthClientRepository};

use super::SharedRepository;

use localid_oauth_client_repository_memory::MemoryOAuthClientRepository;

impl ClientAuthenticationPort for SharedRepository<MemoryOAuthClientRepository> {
    type Error = ();

    fn find_client(&self, client_id: &str) -> Result<Option<OAuthClient>, Self::Error> {
        self.with(|repository| repository.find_by_client_id(client_id))
    }
}
