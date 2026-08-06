use localid_client::ClientId;

/// Represents the authenticated client context.
#[derive(Debug, Clone, Copy)]
pub struct ClientContext {
    client_id: ClientId,
}

impl ClientContext {
    /// Creates a new client context.
    #[must_use]
    pub const fn new(client_id: ClientId) -> Self {
        Self { client_id }
    }

    /// Returns client identifier.
    #[must_use]
    pub const fn client_id(&self) -> ClientId {
        self.client_id
    }
}
