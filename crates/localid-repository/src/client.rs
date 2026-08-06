use localid_client::{Client, ClientId};

/// Repository for Client aggregates.
pub trait ClientRepository {
    /// Repository-specific error.
    type Error;

    /// Finds a client by identifier.
    fn find_by_id(&self, id: ClientId) -> Result<Option<Client>, Self::Error>;

    /// Finds a client by public client identifier.
    fn find_by_client_id(&self, client_id: &str) -> Result<Option<Client>, Self::Error>;

    /// Persists a client.
    fn save(&mut self, client: Client) -> Result<(), Self::Error>;
}
