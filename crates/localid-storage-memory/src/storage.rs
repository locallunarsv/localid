use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use localid_credential::{Credential, CredentialId};
use localid_identity::{Identity, IdentityId};
use localid_password::PasswordMaterial;
use localid_refresh_token::{RefreshToken, RefreshTokenId};
use localid_repository::PasswordMaterialRepository;
use localid_repository::RefreshTokenRepository;
use localid_repository::{
    CredentialRepository, IdentityRepository, SessionRepository, TokenRepository,
};
use localid_session::{Session, SessionId};
use localid_token::{Token, TokenId};

use crate::MemoryStorageError;

#[derive(Debug, Default)]
struct InnerStorage {
    identities: HashMap<IdentityId, Identity>,
    credentials: HashMap<CredentialId, Credential>,
    sessions: HashMap<SessionId, Session>,
    password_materials: HashMap<CredentialId, PasswordMaterial>,
    tokens: HashMap<TokenId, Token>,
    refresh_tokens: HashMap<RefreshTokenId, RefreshToken>,
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

impl RefreshTokenRepository for MemoryStorage {
    type Error = MemoryStorageError;

    fn find_by_id(&self, id: RefreshTokenId) -> Result<Option<RefreshToken>, Self::Error> {
        let storage = self
            .inner
            .read()
            .map_err(|_| MemoryStorageError::LockPoisoned)?;

        Ok(storage.refresh_tokens.get(&id).cloned())
    }

    fn find_by_secret_hash(&self, secret_hash: &str) -> Result<Option<RefreshToken>, Self::Error> {
        let storage = self
            .inner
            .read()
            .map_err(|_| MemoryStorageError::LockPoisoned)?;

        Ok(storage
            .refresh_tokens
            .values()
            .find(|token| token.secret_hash() == secret_hash)
            .cloned())
    }

    fn save(&mut self, token: RefreshToken) -> Result<(), Self::Error> {
        let mut storage = self
            .inner
            .write()
            .map_err(|_| MemoryStorageError::LockPoisoned)?;

        storage.refresh_tokens.insert(token.id(), token);

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

impl PasswordMaterialRepository for MemoryStorage {
    type Error = MemoryStorageError;

    fn find_by_credential_id(
        &self,
        credential_id: CredentialId,
    ) -> Result<Option<PasswordMaterial>, Self::Error> {
        let storage = self
            .inner
            .read()
            .map_err(|_| MemoryStorageError::LockPoisoned)?;

        Ok(storage.password_materials.get(&credential_id).cloned())
    }

    fn save(&mut self, material: PasswordMaterial) -> Result<(), Self::Error> {
        let mut storage = self
            .inner
            .write()
            .map_err(|_| MemoryStorageError::LockPoisoned)?;

        storage
            .password_materials
            .insert(material.credential_id(), material);

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

impl TokenRepository for MemoryStorage {
    type Error = MemoryStorageError;

    fn find_by_id(&self, id: TokenId) -> Result<Option<Token>, Self::Error> {
        let storage = self
            .inner
            .read()
            .map_err(|_| MemoryStorageError::LockPoisoned)?;

        Ok(storage.tokens.get(&id).cloned())
    }
    fn find_by_secret_hash(&self, secret_hash: &str) -> Result<Option<Token>, Self::Error> {
        let storage = self
            .inner
            .read()
            .map_err(|_| MemoryStorageError::LockPoisoned)?;

        Ok(storage
            .tokens
            .values()
            .find(|token| token.secret_hash() == secret_hash)
            .cloned())
    }

    fn save(&mut self, token: Token) -> Result<(), Self::Error> {
        let mut storage = self
            .inner
            .write()
            .map_err(|_| MemoryStorageError::LockPoisoned)?;

        storage.tokens.insert(token.id(), token);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeDelta, TimeZone, Utc};
    use localid_client::ClientId;
    use localid_credential::{Credential, CredentialId, CredentialKind};
    use localid_identity::{Identity, IdentityId};
    use localid_password::PasswordHash;
    use localid_refresh_token::{RefreshToken, RefreshTokenId};
    use localid_repository::RefreshTokenRepository;
    use localid_repository::{
        CredentialRepository, IdentityRepository, SessionRepository, TokenRepository,
    };
    use localid_session::{Session, SessionId};
    use localid_token::{Token, TokenId};

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
            ClientId::new(),
            created_at,
            created_at + TimeDelta::hours(1),
        )
        .expect("session should be valid");

        SessionRepository::save(&mut storage, session).expect("session should be stored");

        let sessions = SessionRepository::find_by_identity_id(&storage, identity_id)
            .expect("session lookup should succeed");

        assert_eq!(sessions.len(), 1);
    }
    #[test]
    fn stores_password_credential_material() {
        use localid_password::PasswordMaterial;
        use localid_repository::PasswordMaterialRepository;

        let mut storage = MemoryStorage::new();
        let credential_id = CredentialId::new();

        let password =
            PasswordMaterial::new(credential_id, PasswordHash::new("$example$hash".to_owned()));

        PasswordMaterialRepository::save(&mut storage, password.clone())
            .expect("password credential should be stored");

        let stored = PasswordMaterialRepository::find_by_credential_id(&storage, credential_id)
            .expect("password credential lookup should succeed");

        assert_eq!(stored, Some(password));
    }
    #[test]
    fn stores_tokens() {
        use localid_token::{Token, TokenId};
        let mut storage = MemoryStorage::new();

        let created_at = Utc
            .with_ymd_and_hms(2026, 8, 2, 0, 0, 0)
            .single()
            .expect("test timestamp should be valid");

        let token = Token::new(
            TokenId::new(),
            localid_session::SessionId::new(),
            "hashed-secret".to_owned(),
            created_at,
            created_at + TimeDelta::hours(1),
        )
        .expect("token should be valid");

        let token_id = token.id();

        TokenRepository::save(&mut storage, token.clone()).expect("token should be stored");

        let stored =
            TokenRepository::find_by_id(&storage, token_id).expect("token lookup should succeed");

        assert_eq!(stored, Some(token));
    }
    #[test]
    fn finds_tokens_by_secret_hash() {
        let mut storage = MemoryStorage::new();

        let created_at = Utc
            .with_ymd_and_hms(2026, 8, 2, 0, 0, 0)
            .single()
            .expect("test timestamp should be valid");

        let token = Token::new(
            TokenId::new(),
            SessionId::new(),
            "hashed-secret".to_owned(),
            created_at,
            created_at + TimeDelta::hours(1),
        )
        .expect("token should be valid");

        TokenRepository::save(&mut storage, token.clone()).expect("token should be stored");

        let stored = TokenRepository::find_by_secret_hash(&storage, "hashed-secret")
            .expect("token lookup should succeed");

        assert_eq!(stored, Some(token));
    }
    #[test]
    fn finds_refresh_tokens_by_secret_hash() {
        let mut storage = MemoryStorage::new();

        let created_at = Utc
            .with_ymd_and_hms(2026, 8, 2, 0, 0, 0)
            .single()
            .expect("timestamp should be valid");

        let refresh_token = RefreshToken::new(
            RefreshTokenId::new(),
            SessionId::new(),
            "refresh-hash".to_owned(),
            created_at,
            created_at + TimeDelta::days(30),
        )
        .expect("refresh token should be valid");

        RefreshTokenRepository::save(&mut storage, refresh_token.clone())
            .expect("refresh token should save");

        let stored = RefreshTokenRepository::find_by_secret_hash(&storage, "refresh-hash")
            .expect("lookup should succeed");

        assert_eq!(stored, Some(refresh_token));
    }
}
