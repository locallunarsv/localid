use chrono::{TimeDelta, Utc};

use localid_authentication::{
    AuthenticatePasswordRequest, DefaultPasswordAuthenticationService,
    PasswordAuthenticationDependencies, PasswordAuthenticationService, SessionFactory,
};
use localid_client::ClientId;
use localid_credential::{Credential, CredentialId, CredentialKind};
use localid_identity::{Identity, IdentityId};
use localid_password::{PasswordHasher, PasswordMaterial, PasswordSecret};
use localid_password_argon2::Argon2PasswordHasher;
use localid_refresh_token_random::RandomRefreshTokenIssuer;
use localid_repository::{
    CredentialRepository, IdentityRepository, PasswordMaterialRepository, SessionRepository,
};
use localid_session::{Session, SessionId};
use localid_storage_memory::MemoryStorage;
use localid_token_random::RandomTokenIssuer;

struct FixedSessionFactory;

impl SessionFactory for FixedSessionFactory {
    type Error = ();

    fn create_session(
        &self,
        identity_id: IdentityId,
        client_id: ClientId,
    ) -> Result<Session, Self::Error> {
        let created_at = Utc::now();

        Session::new(
            SessionId::new(),
            identity_id,
            client_id,
            created_at,
            created_at + TimeDelta::hours(1),
        )
        .map_err(|_| ())
    }
}

#[test]
fn authenticates_password_credential() {
    let mut storage = MemoryStorage::new();

    let identity = Identity::new(IdentityId::new());
    let identity_id = identity.id();

    IdentityRepository::save(&mut storage, identity).expect("Identity should be stored");

    let credential = Credential::new(CredentialId::new(), identity_id, CredentialKind::Password);

    let credential_id = credential.id();

    CredentialRepository::save(&mut storage, credential).expect("Credential should be stored");

    let password = PasswordSecret::new("correct-password").expect("test password should be valid");

    let hasher = Argon2PasswordHasher::new();

    let password_hash =
        PasswordHasher::hash(&hasher, &password).expect("password hashing should succeed");

    let password_material = PasswordMaterial::new(credential_id, password_hash);

    PasswordMaterialRepository::save(&mut storage, password_material)
        .expect("Password Material should be stored");

    let mut service =
        DefaultPasswordAuthenticationService::new(PasswordAuthenticationDependencies {
            identity_repository: storage.clone(),
            credential_repository: storage.clone(),
            password_material_repository: storage.clone(),
            session_repository: storage.clone(),
            token_repository: storage.clone(),
            refresh_token_repository: storage.clone(),
            password_verifier: Argon2PasswordHasher::new(),
            session_factory: FixedSessionFactory,
            token_issuer: RandomTokenIssuer::new(),
            refresh_token_issuer: RandomRefreshTokenIssuer::new(),
        });

    let request = AuthenticatePasswordRequest::new(ClientId::new(), credential_id, password);

    let result = service
        .authenticate_password(request)
        .expect("password authentication should succeed");

    let session = result.session();

    assert_eq!(session.identity_id(), identity_id);
    assert!(session.is_active());

    let stored_session = SessionRepository::find_by_id(&storage, session.id())
        .expect("Session lookup should succeed")
        .expect("authenticated Session should be stored");

    assert_eq!(stored_session, *session);
}

#[test]
fn rejects_invalid_password() {
    let mut storage = MemoryStorage::new();

    let identity = Identity::new(IdentityId::new());
    let identity_id = identity.id();

    IdentityRepository::save(&mut storage, identity).expect("Identity should be stored");

    let credential = Credential::new(CredentialId::new(), identity_id, CredentialKind::Password);

    let credential_id = credential.id();

    CredentialRepository::save(&mut storage, credential).expect("Credential should be stored");

    let password = PasswordSecret::new("correct-password").expect("test password should be valid");

    let hasher = Argon2PasswordHasher::new();

    let password_hash =
        PasswordHasher::hash(&hasher, &password).expect("password hashing should succeed");

    PasswordMaterialRepository::save(
        &mut storage,
        PasswordMaterial::new(credential_id, password_hash),
    )
    .expect("Password Material should be stored");

    let mut service =
        DefaultPasswordAuthenticationService::new(PasswordAuthenticationDependencies {
            identity_repository: storage.clone(),
            credential_repository: storage.clone(),
            password_material_repository: storage.clone(),
            session_repository: storage.clone(),
            token_repository: storage.clone(),
            refresh_token_repository: storage.clone(),
            password_verifier: Argon2PasswordHasher::new(),
            session_factory: FixedSessionFactory,
            token_issuer: RandomTokenIssuer::new(),
            refresh_token_issuer: RandomRefreshTokenIssuer::new(),
        });

    let wrong_password =
        PasswordSecret::new("wrong-password").expect("test password should be valid");

    let request = AuthenticatePasswordRequest::new(ClientId::new(), credential_id, wrong_password);

    let result = service.authenticate_password(request);

    assert!(result.is_err());
}

#[test]
fn rejects_disabled_identity() {
    let mut storage = MemoryStorage::new();

    let mut identity = Identity::new(IdentityId::new());

    identity.disable().expect("identity should be disabled");

    let identity_id = identity.id();

    IdentityRepository::save(&mut storage, identity).expect("Identity should be stored");

    let credential = Credential::new(CredentialId::new(), identity_id, CredentialKind::Password);

    let credential_id = credential.id();

    CredentialRepository::save(&mut storage, credential).expect("Credential should be stored");

    let password = PasswordSecret::new("password").expect("test password should be valid");

    let hasher = Argon2PasswordHasher::new();

    let password_hash =
        PasswordHasher::hash(&hasher, &password).expect("password hashing should succeed");

    PasswordMaterialRepository::save(
        &mut storage,
        PasswordMaterial::new(credential_id, password_hash),
    )
    .expect("Password Material should be stored");

    let mut service =
        DefaultPasswordAuthenticationService::new(PasswordAuthenticationDependencies {
            identity_repository: storage.clone(),
            credential_repository: storage.clone(),
            password_material_repository: storage.clone(),
            session_repository: storage.clone(),
            token_repository: storage.clone(),
            refresh_token_repository: storage.clone(),
            password_verifier: Argon2PasswordHasher::new(),
            session_factory: FixedSessionFactory,
            token_issuer: RandomTokenIssuer::new(),
            refresh_token_issuer: RandomRefreshTokenIssuer::new(),
        });

    let request = AuthenticatePasswordRequest::new(ClientId::new(), credential_id, password);

    let result = service.authenticate_password(request);

    assert!(result.is_err());
}

#[test]
fn rejects_disabled_credential() {
    let mut storage = MemoryStorage::new();

    let identity = Identity::new(IdentityId::new());
    let identity_id = identity.id();

    IdentityRepository::save(&mut storage, identity).expect("Identity should be stored");

    let mut credential =
        Credential::new(CredentialId::new(), identity_id, CredentialKind::Password);

    credential.disable().expect("credential should be disabled");

    let credential_id = credential.id();

    CredentialRepository::save(&mut storage, credential).expect("Credential should be stored");

    let mut service =
        DefaultPasswordAuthenticationService::new(PasswordAuthenticationDependencies {
            identity_repository: storage.clone(),
            credential_repository: storage.clone(),
            password_material_repository: storage.clone(),
            session_repository: storage.clone(),
            token_repository: storage.clone(),
            refresh_token_repository: storage.clone(),
            password_verifier: Argon2PasswordHasher::new(),
            session_factory: FixedSessionFactory,
            token_issuer: RandomTokenIssuer::new(),
            refresh_token_issuer: RandomRefreshTokenIssuer::new(),
        });

    let password = PasswordSecret::new("password").expect("test password should be valid");

    let request = AuthenticatePasswordRequest::new(ClientId::new(), credential_id, password);

    let result = service.authenticate_password(request);

    assert!(result.is_err());
}
