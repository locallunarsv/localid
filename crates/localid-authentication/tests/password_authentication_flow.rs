use chrono::{TimeDelta, TimeZone, Utc};
use localid_authentication::{
    AuthenticatePasswordRequest, DefaultPasswordAuthenticationService,
    PasswordAuthenticationService, SessionFactory,
};
use localid_credential::{Credential, CredentialId, CredentialKind};
use localid_identity::{Identity, IdentityId};
use localid_password::{PasswordHasher, PasswordMaterial, PasswordSecret};
use localid_password_argon2::Argon2PasswordHasher;
use localid_repository::{
    CredentialRepository, IdentityRepository, PasswordMaterialRepository, SessionRepository,
};
use localid_session::{Session, SessionId};
use localid_storage_memory::MemoryStorage;
use localid_token_random::RandomTokenIssuer;

struct FixedSessionFactory;

impl SessionFactory for FixedSessionFactory {
    type Error = ();

    fn create_session(&self, identity_id: IdentityId) -> Result<Session, Self::Error> {
        let created_at = Utc
            .with_ymd_and_hms(2026, 8, 2, 0, 0, 0)
            .single()
            .expect("test timestamp should be valid");

        Session::new(
            SessionId::new(),
            identity_id,
            created_at,
            created_at + TimeDelta::hours(1),
        )
        .map_err(|_| ())
    }
}

#[test]
fn authenticates_password_credential() {
    // Arrange

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

    let mut service = DefaultPasswordAuthenticationService::new(
        storage.clone(),
        storage.clone(),
        storage.clone(),
        storage.clone(),
        hasher,
        FixedSessionFactory,
        RandomTokenIssuer::new(),
    );

    let request = AuthenticatePasswordRequest::new(credential_id, password);

    // Act

    let result = service
        .authenticate_password(request)
        .expect("password authentication should succeed");

    // Assert

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
    use localid_authentication::AuthenticationError;
    // Arrange

    let mut storage = MemoryStorage::new();

    let identity = Identity::new(IdentityId::new());
    let identity_id = identity.id();

    IdentityRepository::save(&mut storage, identity).expect("Identity should be stored");

    let credential = Credential::new(CredentialId::new(), identity_id, CredentialKind::Password);
    let credential_id = credential.id();

    CredentialRepository::save(&mut storage, credential).expect("Credential should be stored");

    let correct_password =
        PasswordSecret::new("correct-password").expect("password should be valid");

    let wrong_password = PasswordSecret::new("wrong-password").expect("password should be valid");

    let hasher = Argon2PasswordHasher::new();

    let password_hash =
        PasswordHasher::hash(&hasher, &correct_password).expect("password hashing should succeed");

    let material = PasswordMaterial::new(credential_id, password_hash);

    PasswordMaterialRepository::save(&mut storage, material)
        .expect("Password Material should be stored");

    let mut service = DefaultPasswordAuthenticationService::new(
        storage.clone(),
        storage.clone(),
        storage.clone(),
        storage.clone(),
        hasher,
        FixedSessionFactory,
        RandomTokenIssuer::new(),
    );

    let request = AuthenticatePasswordRequest::new(credential_id, wrong_password);

    // Act

    let result = service.authenticate_password(request);

    // Assert

    assert_eq!(result, Err(AuthenticationError::InvalidPassword));
}

#[test]
fn rejects_disabled_credential() {
    use localid_authentication::AuthenticationError;
    // Arrange

    let mut storage = MemoryStorage::new();

    let identity = Identity::new(IdentityId::new());
    let identity_id = identity.id();

    IdentityRepository::save(&mut storage, identity).expect("Identity should be stored");

    let mut credential =
        Credential::new(CredentialId::new(), identity_id, CredentialKind::Password);

    let credential_id = credential.id();

    credential.disable().expect("credential should be disabled");

    CredentialRepository::save(&mut storage, credential).expect("Credential should be stored");

    let password = PasswordSecret::new("correct-password").expect("password should be valid");

    let hasher = Argon2PasswordHasher::new();

    let password_hash =
        PasswordHasher::hash(&hasher, &password).expect("password hashing should succeed");

    let material = PasswordMaterial::new(credential_id, password_hash);

    PasswordMaterialRepository::save(&mut storage, material)
        .expect("Password Material should be stored");

    let mut service = DefaultPasswordAuthenticationService::new(
        storage.clone(),
        storage.clone(),
        storage.clone(),
        storage.clone(),
        hasher,
        FixedSessionFactory,
        RandomTokenIssuer::new(),
    );

    let request = AuthenticatePasswordRequest::new(credential_id, password);

    // Act

    let result = service.authenticate_password(request);

    // Assert

    assert_eq!(result, Err(AuthenticationError::CredentialUnavailable));
}

#[test]
fn rejects_disabled_identity() {
    use localid_authentication::AuthenticationError;
    // Arrange

    let mut storage = MemoryStorage::new();

    let mut identity = Identity::new(IdentityId::new());
    let identity_id = identity.id();

    identity.disable().expect("identity should be disabled");

    IdentityRepository::save(&mut storage, identity).expect("Identity should be stored");

    let credential = Credential::new(CredentialId::new(), identity_id, CredentialKind::Password);

    let credential_id = credential.id();

    CredentialRepository::save(&mut storage, credential).expect("Credential should be stored");

    let password = PasswordSecret::new("correct-password").expect("password should be valid");

    let hasher = Argon2PasswordHasher::new();

    let password_hash =
        PasswordHasher::hash(&hasher, &password).expect("password hashing should succeed");

    let material = PasswordMaterial::new(credential_id, password_hash);

    PasswordMaterialRepository::save(&mut storage, material)
        .expect("Password Material should be stored");

    let mut service = DefaultPasswordAuthenticationService::new(
        storage.clone(),
        storage.clone(),
        storage.clone(),
        storage.clone(),
        hasher,
        FixedSessionFactory,
        RandomTokenIssuer::new(),
    );

    let request = AuthenticatePasswordRequest::new(credential_id, password);

    // Act

    let result = service.authenticate_password(request);

    // Assert

    assert_eq!(result, Err(AuthenticationError::IdentityUnavailable));
}

#[test]
fn rejects_missing_password_material() {
    use localid_authentication::AuthenticationError;
    // Arrange

    let mut storage = MemoryStorage::new();

    let identity = Identity::new(IdentityId::new());
    let identity_id = identity.id();

    IdentityRepository::save(&mut storage, identity).expect("Identity should be stored");

    let credential = Credential::new(CredentialId::new(), identity_id, CredentialKind::Password);

    let credential_id = credential.id();

    CredentialRepository::save(&mut storage, credential).expect("Credential should be stored");

    let password = PasswordSecret::new("correct-password").expect("password should be valid");

    let hasher = Argon2PasswordHasher::new();

    let mut service = DefaultPasswordAuthenticationService::new(
        storage.clone(),
        storage.clone(),
        storage.clone(),
        storage.clone(),
        hasher,
        FixedSessionFactory,
        RandomTokenIssuer::new(),
    );

    let request = AuthenticatePasswordRequest::new(credential_id, password);

    // Act

    let result = service.authenticate_password(request);

    // Assert

    assert_eq!(result, Err(AuthenticationError::PasswordMaterialNotFound));
}
