use super::{OAuthClientError, OAuthClientId, OAuthClientLifecycleState};

/// OAuth client application aggregate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OAuthClient {
    id: OAuthClientId,
    client_id: String,
    name: String,
    secret_hash: String,
    redirect_uris: Vec<String>,
    state: OAuthClientLifecycleState,
}

impl OAuthClient {
    /// Creates a new active OAuth client.
    #[must_use]
    pub fn new(
        id: OAuthClientId,
        client_id: impl Into<String>,
        name: impl Into<String>,
        secret_hash: impl Into<String>,
        redirect_uris: Vec<String>,
    ) -> Self {
        Self {
            id,
            client_id: client_id.into(),
            name: name.into(),
            secret_hash: secret_hash.into(),
            redirect_uris,
            state: OAuthClientLifecycleState::Active,
        }
    }

    /// Returns internal identifier.
    #[must_use]
    pub const fn id(&self) -> OAuthClientId {
        self.id
    }

    /// Returns public client identifier.
    #[must_use]
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Returns client name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns secret hash.
    #[must_use]
    pub fn secret_hash(&self) -> &str {
        &self.secret_hash
    }

    /// Returns registered redirect URIs.
    #[must_use]
    pub fn redirect_uris(&self) -> &[String] {
        &self.redirect_uris
    }

    /// Returns lifecycle state.
    #[must_use]
    pub const fn state(&self) -> OAuthClientLifecycleState {
        self.state
    }

    /// Disables client.
    pub fn disable(&mut self) -> Result<(), OAuthClientError> {
        if self.state == OAuthClientLifecycleState::Deleted {
            return Err(OAuthClientError::AlreadyDeleted);
        }

        self.state = OAuthClientLifecycleState::Disabled;

        Ok(())
    }

    /// Activates client.
    pub fn activate(&mut self) -> Result<(), OAuthClientError> {
        if self.state == OAuthClientLifecycleState::Deleted {
            return Err(OAuthClientError::AlreadyDeleted);
        }

        self.state = OAuthClientLifecycleState::Active;

        Ok(())
    }

    /// Deletes client.
    pub fn delete(&mut self) -> Result<(), OAuthClientError> {
        if self.state == OAuthClientLifecycleState::Deleted {
            return Err(OAuthClientError::AlreadyDeleted);
        }

        self.state = OAuthClientLifecycleState::Deleted;

        Ok(())
    }

    /// Checks whether redirect URI is registered.
    #[must_use]
    pub fn allows_redirect_uri(&self, redirect_uri: &str) -> bool {
        self.redirect_uris
            .iter()
            .any(|registered| registered == redirect_uri)
    }
}
