use super::{ClientError, ClientId, ClientLifecycleState};

/// Client application aggregate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Client {
    id: ClientId,
    client_id: String,
    name: String,
    state: ClientLifecycleState,
}

impl Client {
    /// Creates a new active client.
    #[must_use]
    pub fn new(id: ClientId, client_id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id,
            client_id: client_id.into(),
            name: name.into(),
            state: ClientLifecycleState::Active,
        }
    }

    /// Returns client identifier.
    #[must_use]
    pub const fn id(&self) -> ClientId {
        self.id
    }

    /// Returns public client id.
    #[must_use]
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Returns client name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns lifecycle state.
    #[must_use]
    pub const fn state(&self) -> ClientLifecycleState {
        self.state
    }

    /// Disables client.
    pub fn disable(&mut self) -> Result<(), ClientError> {
        if self.state == ClientLifecycleState::Deleted {
            return Err(ClientError::AlreadyDeleted);
        }

        self.state = ClientLifecycleState::Disabled;

        Ok(())
    }

    /// Activates client.
    pub fn activate(&mut self) -> Result<(), ClientError> {
        if self.state == ClientLifecycleState::Deleted {
            return Err(ClientError::AlreadyDeleted);
        }

        self.state = ClientLifecycleState::Active;

        Ok(())
    }

    /// Returns whether this client can be used.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.state.is_active()
    }

    /// Returns whether this client has been disabled.
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        self.state.is_disabled()
    }

    /// Returns whether this client has been deleted.
    #[must_use]
    pub const fn is_deleted(&self) -> bool {
        self.state.is_deleted()
    }

    /// Deletes client.
    pub fn delete(&mut self) -> Result<(), ClientError> {
        if self.state == ClientLifecycleState::Deleted {
            return Err(ClientError::AlreadyDeleted);
        }

        self.state = ClientLifecycleState::Deleted;

        Ok(())
    }
}
