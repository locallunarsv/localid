use std::collections::HashMap;

use localid_client::{Client, ClientId};
use localid_repository::ClientRepository;

use crate::MemoryRepositoryError;

/// In-memory client repository.
#[derive(Debug, Default)]
pub struct MemoryClientRepository {
    clients: HashMap<ClientId, Client>,
    client_index: HashMap<String, ClientId>,
}

impl MemoryClientRepository {
    /// Creates empty repository.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl ClientRepository for MemoryClientRepository {
    type Error = MemoryRepositoryError;

    fn find_by_id(&self, id: ClientId) -> Result<Option<Client>, Self::Error> {
        Ok(self.clients.get(&id).cloned())
    }

    fn find_by_client_id(&self, client_id: &str) -> Result<Option<Client>, Self::Error> {
        let Some(id) = self.client_index.get(client_id) else {
            return Ok(None);
        };

        Ok(self.clients.get(id).cloned())
    }

    fn save(&mut self, client: Client) -> Result<(), Self::Error> {
        self.client_index
            .insert(client.client_id().to_owned(), client.id());

        self.clients.insert(client.id(), client);

        Ok(())
    }
}
