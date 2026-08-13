use std::collections::HashMap;

use localid_oauth_client::{OAuthClient, OAuthClientId, OAuthClientRepository};

#[derive(Debug, Default, Clone)]
pub struct MemoryOAuthClientRepository {
    clients: HashMap<OAuthClientId, OAuthClient>,
}

impl MemoryOAuthClientRepository {
    #[must_use]
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }
}

impl OAuthClientRepository for MemoryOAuthClientRepository {
    type Error = ();

    fn save(&mut self, client: OAuthClient) -> Result<(), Self::Error> {
        self.clients.insert(client.id(), client);

        Ok(())
    }

    fn find_by_id(&self, id: OAuthClientId) -> Result<Option<OAuthClient>, Self::Error> {
        Ok(self.clients.get(&id).cloned())
    }

    fn find_by_client_id(&self, client_id: &str) -> Result<Option<OAuthClient>, Self::Error> {
        Ok(self
            .clients
            .values()
            .find(|client| client.client_id() == client_id)
            .cloned())
    }
    fn find_all(&self) -> Result<Vec<OAuthClient>, Self::Error> {
        Ok(self.clients.values().cloned().collect())
    }
}
