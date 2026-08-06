use localid_client::{Client, ClientId};
use localid_repository::ClientRepository;

use super::SharedRepository;

impl<T> ClientRepository for SharedRepository<T>
where
    T: ClientRepository,
{
    type Error = T::Error;

    fn find_by_id(&self, id: ClientId) -> Result<Option<Client>, Self::Error> {
        self.with(|repository| repository.find_by_id(id))
    }

    fn find_by_client_id(&self, client_id: &str) -> Result<Option<Client>, Self::Error> {
        self.with(|repository| repository.find_by_client_id(client_id))
    }

    fn save(&mut self, client: Client) -> Result<(), Self::Error> {
        self.with_mut(|repository| repository.save(client))
    }
}
