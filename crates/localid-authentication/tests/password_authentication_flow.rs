use chrono::{ TimeDelta, Utc };

use localid_authentication::{
    AuthenticatePasswordRequest,
    AuthenticateResult,
    DefaultPasswordAuthenticationService,
    PasswordAuthenticationDependencies,
    PasswordAuthenticationService,
    TokenIssuanceService,
};

use localid_client::ClientId;
use localid_credential::{ Credential, CredentialId, CredentialKind };
use localid_identity::{ Identity, IdentityId };

use localid_password::{ PasswordHasher, PasswordMaterial, PasswordSecret };

use localid_password_argon2::Argon2PasswordHasher;

use localid_repository::{ CredentialRepository, IdentityRepository, PasswordMaterialRepository };

use localid_session::{ Session, SessionId };

use localid_storage_memory::MemoryStorage;

use localid_refresh_token::{ RefreshToken, RefreshTokenId };
use localid_refresh_token_random::IssuedRefreshToken;

use localid_token::Token;
use localid_token_random::IssuedToken;

struct FakeTokenIssuanceService;

impl TokenIssuanceService for FakeTokenIssuanceService {
    type Error = localid_authentication::AuthenticationError;

    fn issue(
        &mut self,
        identity_id: IdentityId,
        client_id: ClientId
    ) -> Result<AuthenticateResult, Self::Error> {
        let created_at = Utc::now();

        let session = Session::new(
            SessionId::new(),
            identity_id,
            client_id,
            created_at,
            created_at + TimeDelta::hours(1)
        ).map_err(|_| Self::Error::SessionCreationFailure)?;

        let token = Token::new(
            localid_token::TokenId::new(),
            session.id(),
            "token-hash".to_string(),
            session.created_at(),
            session.expires_at()
        ).map_err(|_| Self::Error::TokenCreationFailure)?;

        let issued_token = IssuedToken::new(token, "access-token".to_string());

        let refresh_token = RefreshToken::new(
            RefreshTokenId::new(),
            session.id(),
            "refresh-hash".to_string(),
            session.created_at(),
            session.expires_at()
        ).map_err(|_| Self::Error::TokenCreationFailure)?;

        let issued_refresh_token = IssuedRefreshToken::new(
            refresh_token,
            "refresh-token".to_string()
        );

        Ok(AuthenticateResult::new(session, issued_token, issued_refresh_token))
    }
}

fn create_service(
    storage: &MemoryStorage
) -> DefaultPasswordAuthenticationService<
    MemoryStorage,
    MemoryStorage,
    MemoryStorage,
    Argon2PasswordHasher,
    FakeTokenIssuanceService
> {
    DefaultPasswordAuthenticationService::new(PasswordAuthenticationDependencies {
        identity_repository: storage.clone(),
        credential_repository: storage.clone(),
        password_material_repository: storage.clone(),
        password_verifier: Argon2PasswordHasher::new(),
        token_issuance_service: FakeTokenIssuanceService,
    })
}

#[test]
fn authenticates_password_credential() {
    let mut storage = MemoryStorage::new();

    let identity = Identity::new(IdentityId::new());
    let identity_id = identity.id();

    IdentityRepository::save(&mut storage, identity).expect("identity should be stored");

    let credential = Credential::new(CredentialId::new(), identity_id, CredentialKind::Password);

    let credential_id = credential.id();

    CredentialRepository::save(&mut storage, credential).expect("credential should be stored");

    let password = PasswordSecret::new("correct-password").expect("password should be valid");

    let hasher = Argon2PasswordHasher::new();

    let password_hash = PasswordHasher::hash(&hasher, &password).expect("hash should succeed");

    PasswordMaterialRepository::save(
        &mut storage,
        PasswordMaterial::new(credential_id, password_hash)
    ).expect("password material should be stored");

    let mut service = create_service(&storage);

    let request = AuthenticatePasswordRequest::new(ClientId::new(), credential_id, password);

    let result = service.authenticate_password(request).expect("authentication should succeed");

    assert_eq!(result.session().identity_id(), identity_id);
}

#[test]
fn rejects_invalid_password() {
    let mut storage = MemoryStorage::new();

    let identity = Identity::new(IdentityId::new());
    let identity_id = identity.id();

    IdentityRepository::save(&mut storage, identity).unwrap();

    let credential = Credential::new(CredentialId::new(), identity_id, CredentialKind::Password);

    let credential_id = credential.id();

    CredentialRepository::save(&mut storage, credential).unwrap();

    let password = PasswordSecret::new("correct-password").unwrap();

    let hash = Argon2PasswordHasher::new().hash(&password).unwrap();

    PasswordMaterialRepository::save(
        &mut storage,
        PasswordMaterial::new(credential_id, hash)
    ).unwrap();

    let mut service = create_service(&storage);

    let wrong = PasswordSecret::new("wrong-password").unwrap();

    let request = AuthenticatePasswordRequest::new(ClientId::new(), credential_id, wrong);

    assert!(service.authenticate_password(request).is_err());
}

#[test]
fn rejects_disabled_identity() {
    let mut storage = MemoryStorage::new();

    let mut identity = Identity::new(IdentityId::new());

    identity.disable().unwrap();

    let identity_id = identity.id();

    IdentityRepository::save(&mut storage, identity).unwrap();

    let credential = Credential::new(CredentialId::new(), identity_id, CredentialKind::Password);

    let credential_id = credential.id();

    CredentialRepository::save(&mut storage, credential).unwrap();

    let password = PasswordSecret::new("password").unwrap();

    let hash = Argon2PasswordHasher::new().hash(&password).unwrap();

    PasswordMaterialRepository::save(
        &mut storage,
        PasswordMaterial::new(credential_id, hash)
    ).unwrap();

    let mut service = create_service(&storage);

    let request = AuthenticatePasswordRequest::new(ClientId::new(), credential_id, password);

    assert!(service.authenticate_password(request).is_err());
}

#[test]
fn rejects_disabled_credential() {
    let mut storage = MemoryStorage::new();

    let identity = Identity::new(IdentityId::new());

    let identity_id = identity.id();

    IdentityRepository::save(&mut storage, identity).unwrap();

    let mut credential = Credential::new(
        CredentialId::new(),
        identity_id,
        CredentialKind::Password
    );

    credential.disable().unwrap();

    let credential_id = credential.id();

    CredentialRepository::save(&mut storage, credential).unwrap();

    let mut service = create_service(&storage);

    let password = PasswordSecret::new("password").unwrap();

    let request = AuthenticatePasswordRequest::new(ClientId::new(), credential_id, password);

    assert!(service.authenticate_password(request).is_err());
}
