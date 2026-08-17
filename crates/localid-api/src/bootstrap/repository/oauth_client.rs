use localid_oauth_client::{OAuthClient, OAuthClientId, OAuthClientRepository};

use super::SharedRepository;

use localid_database_postgres::{DatabaseError, PostgresOAuthClientRepository};
use localid_oauth_client_repository_memory::MemoryOAuthClientRepository;

impl OAuthClientRepository for SharedRepository<MemoryOAuthClientRepository> {
    type Error = ();

    fn save(&mut self, client: OAuthClient) -> Result<(), Self::Error> {
        self.with_mut(|repository| repository.save(client))
    }

    fn find_by_id(&self, id: OAuthClientId) -> Result<Option<OAuthClient>, Self::Error> {
        self.with(|repository| repository.find_by_id(id))
    }

    fn find_by_client_id(&self, client_id: &str) -> Result<Option<OAuthClient>, Self::Error> {
        self.with(|repository| repository.find_by_client_id(client_id))
    }

    fn find_all(&self) -> Result<Vec<OAuthClient>, Self::Error> {
        self.with(|repository| repository.find_all())
    }
}

impl OAuthClientRepository for SharedRepository<PostgresOAuthClientRepository> {
    type Error = DatabaseError;

    fn save(&mut self, client: OAuthClient) -> Result<(), Self::Error> {
        self.with_mut(|repository| repository.save(client))
    }

    fn find_by_id(&self, id: OAuthClientId) -> Result<Option<OAuthClient>, Self::Error> {
        self.with(|repository| repository.find_by_id(id))
    }

    fn find_by_client_id(&self, client_id: &str) -> Result<Option<OAuthClient>, Self::Error> {
        self.with(|repository| repository.find_by_client_id(client_id))
    }

    fn find_all(&self) -> Result<Vec<OAuthClient>, Self::Error> {
        self.with(|repository| repository.find_all())
    }
}
