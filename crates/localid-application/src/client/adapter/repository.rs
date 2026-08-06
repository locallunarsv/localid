use localid_client::{Client, ClientId};
use localid_repository::ClientRepository;

use super::super::ClientPort;

/// Repository adapter for client access.
#[derive(Debug, Clone, Copy)]
pub struct ClientRepositoryAdapter<R> {
    repository: R,
}

impl<R> ClientRepositoryAdapter<R> {
    /// Creates a new client repository adapter.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> ClientPort for ClientRepositoryAdapter<R>
where
    R: ClientRepository,
{
    type Error = R::Error;

    fn find_by_client_id(&self, client_id: &str) -> Result<Option<Client>, Self::Error> {
        self.repository.find_by_client_id(client_id)
    }

    fn find_by_id(&self, id: ClientId) -> Result<Option<Client>, Self::Error> {
        self.repository.find_by_id(id)
    }
}
