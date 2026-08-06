use localid_client::Client;
use localid_client::ClientId;

/// Port for client lookup operations.
pub trait ClientPort {
    /// Application error.
    type Error;

    /// Finds client by public identifier.
    fn find_by_client_id(&self, client_id: &str) -> Result<Option<Client>, Self::Error>;

    /// Finds client by identifier.
    fn find_by_id(&self, id: ClientId) -> Result<Option<Client>, Self::Error>;
}
