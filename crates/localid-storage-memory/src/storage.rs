use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use localid_credential::{Credential, CredentialId};
use localid_identity::{Identity, IdentityId};
use localid_repository::{CredentialRepository, IdentityRepository, SessionRepository};
use localid_session::{Session, SessionId};

use crate::MemoryStorageError;

#[derive(Debug, Default)]
struct InnerStorage {
    identities: HashMap<IdentityId, Identity>,
    credentials: HashMap<CredentialId, Credential>,
    sessions: HashMap<SessionId, Session>,
}

/// Shared in-memory implementation of LocalID repository contracts.
///
/// Cloning `MemoryStorage` creates another handle to the same underlying state.
#[derive(Debug, Clone, Default)]
pub struct MemoryStorage {
    inner: Arc<RwLock<InnerStorage>>,
}

impl MemoryStorage {
    /// Creates an empty in-memory storage.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl IdentityRepository for MemoryStorage {
    type Error = MemoryStorageError;

    fn find_by_id(&self, id: IdentityId) -> Result<Option<Identity>, Self::Error> {
        let storage = self
            .inner
            .read()
            .map_err(|_| MemoryStorageError::LockPoisoned)?;

        Ok(storage.identities.get(&id).cloned())
    }

    fn save(&mut self, identity: Identity) -> Result<(), Self::Error> {
        let mut storage = self
            .inner
            .write()
            .map_err(|_| MemoryStorageError::LockPoisoned)?;

        storage.identities.insert(identity.id(), identity);

        Ok(())
    }
}

impl CredentialRepository for MemoryStorage {
    type Error = MemoryStorageError;

    fn find_by_id(&self, id: CredentialId) -> Result<Option<Credential>, Self::Error> {
        let storage = self
            .inner
            .read()
            .map_err(|_| MemoryStorageError::LockPoisoned)?;

        Ok(storage.credentials.get(&id).cloned())
    }

    fn find_by_identity_id(&self, identity_id: IdentityId) -> Result<Vec<Credential>, Self::Error> {
        let storage = self
            .inner
            .read()
            .map_err(|_| MemoryStorageError::LockPoisoned)?;

        Ok(storage
            .credentials
            .values()
            .filter(|credential| credential.identity_id() == identity_id)
            .cloned()
            .collect())
    }

    fn save(&mut self, credential: Credential) -> Result<(), Self::Error> {
        let mut storage = self
            .inner
            .write()
            .map_err(|_| MemoryStorageError::LockPoisoned)?;

        storage.credentials.insert(credential.id(), credential);

        Ok(())
    }
}

impl SessionRepository for MemoryStorage {
    type Error = MemoryStorageError;

    fn find_by_id(&self, id: SessionId) -> Result<Option<Session>, Self::Error> {
        let storage = self
            .inner
            .read()
            .map_err(|_| MemoryStorageError::LockPoisoned)?;

        Ok(storage.sessions.get(&id).cloned())
    }

    fn find_by_identity_id(&self, identity_id: IdentityId) -> Result<Vec<Session>, Self::Error> {
        let storage = self
            .inner
            .read()
            .map_err(|_| MemoryStorageError::LockPoisoned)?;

        Ok(storage
            .sessions
            .values()
            .filter(|session| session.identity_id() == identity_id)
            .cloned()
            .collect())
    }

    fn save(&mut self, session: Session) -> Result<(), Self::Error> {
        let mut storage = self
            .inner
            .write()
            .map_err(|_| MemoryStorageError::LockPoisoned)?;

        storage.sessions.insert(session.id(), session);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeDelta, TimeZone, Utc};
    use localid_credential::{Credential, CredentialId, CredentialKind};
    use localid_identity::{Identity, IdentityId};
    use localid_repository::{CredentialRepository, IdentityRepository, SessionRepository};
    use localid_session::{Session, SessionId};

    use super::MemoryStorage;

    #[test]
    fn cloned_storage_handles_share_identity_state() {
        let mut writer = MemoryStorage::new();
        let reader = writer.clone();

        let identity = Identity::new(IdentityId::new());
        let identity_id = identity.id();

        IdentityRepository::save(&mut writer, identity).expect("identity should be stored");

        let stored = IdentityRepository::find_by_id(&reader, identity_id)
            .expect("identity lookup should succeed");

        assert!(stored.is_some());
    }

    #[test]
    fn finds_credentials_by_identity() {
        let mut storage = MemoryStorage::new();
        let identity_id = IdentityId::new();

        let credential =
            Credential::new(CredentialId::new(), identity_id, CredentialKind::Password);

        CredentialRepository::save(&mut storage, credential).expect("credential should be stored");

        let credentials = CredentialRepository::find_by_identity_id(&storage, identity_id)
            .expect("credential lookup should succeed");

        assert_eq!(credentials.len(), 1);
    }

    #[test]
    fn finds_sessions_by_identity() {
        let mut storage = MemoryStorage::new();
        let identity_id = IdentityId::new();

        let created_at = Utc
            .with_ymd_and_hms(2026, 8, 2, 0, 0, 0)
            .single()
            .expect("test timestamp should be valid");

        let session = Session::new(
            SessionId::new(),
            identity_id,
            created_at,
            created_at + TimeDelta::hours(1),
        )
        .expect("session should be valid");

        SessionRepository::save(&mut storage, session).expect("session should be stored");

        let sessions = SessionRepository::find_by_identity_id(&storage, identity_id)
            .expect("session lookup should succeed");

        assert_eq!(sessions.len(), 1);
    }
}
