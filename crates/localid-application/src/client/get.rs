use localid_client::Client;

use super::{ClientApplicationError, ClientPort, FindClientQuery};

/// Use case for finding a client.
/// Use case for finding a client.
#[derive(Debug)]
pub struct GetClientUseCase<P> {
    port: P,
}

impl<P> GetClientUseCase<P> {
    /// Creates a new client lookup use case.
    #[must_use]
    pub const fn new(port: P) -> Self {
        Self { port }
    }
}

impl<P> GetClientUseCase<P>
where
    P: ClientPort,
{
    /// Executes client lookup.
    pub fn execute(&self, query: FindClientQuery) -> Result<Client, ClientApplicationError> {
        let client = self
            .port
            .find_by_client_id(query.client_id())
            .map_err(|_| ClientApplicationError::RepositoryFailure)?
            .ok_or(ClientApplicationError::ClientNotFound)?;

        if !client.is_active() {
            return Err(ClientApplicationError::ClientUnavailable);
        }

        Ok(client)
    }
}
