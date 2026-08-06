/// Query for finding a client by client identifier.
#[derive(Debug, Clone)]
pub struct FindClientQuery {
    client_id: String,
}

impl FindClientQuery {
    /// Creates a new query.
    #[must_use]
    pub fn new(client_id: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
        }
    }

    /// Returns client identifier.
    #[must_use]
    pub fn client_id(&self) -> &str {
        &self.client_id
    }
}
