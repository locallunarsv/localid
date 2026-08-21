use uuid::Uuid;

use localid_client::{Client, ClientId};
use localid_credential::{Credential, CredentialId, CredentialKind};
use localid_identity::{Identity, IdentityId};
use localid_oauth_client::{OAuthClient, OAuthClientId, OAuthClientRepository};
use localid_password::{PasswordHasher, PasswordMaterial, PasswordSecret};
use localid_password_argon2::Argon2PasswordHasher;
use localid_permission::Permission;
use localid_repository::{
    ClientRepository, CredentialRepository, IdentityRepository, IdentityRoleRepository,
    PasswordMaterialRepository,
};
use localid_role::Role;

use localid_crypto::hash_secret;

const DEMO_IDENTITY_UUID: Uuid = Uuid::from_u128(0x00000000000070008000000000000001);

const DEMO_CREDENTIAL_UUID: Uuid = Uuid::from_u128(0x00000000000070008000000000000002);

const DEMO_CLIENT_UUID: Uuid = Uuid::from_u128(0x00000000000070008000000000000003);

const DEMO_OAUTH_CLIENT_UUID: Uuid = Uuid::from_u128(0x00000000000070008000000000000004);

const DEMO_OAUTH_OTHER_CLIENT_UUID: Uuid = Uuid::from_u128(0x00000000000070008000000000000005);

/// Seeds a demo password identity.
///
/// Returns credential identifier and identity identifier.
pub fn seed_demo_identity<IR, CR, PR, RR>(
    identity_repository: &mut IR,
    credential_repository: &mut CR,
    password_material_repository: &mut PR,
    identity_role_repository: &mut RR,
) -> (CredentialId, IdentityId)
where
    IR: IdentityRepository,
    CR: CredentialRepository,
    PR: PasswordMaterialRepository,
    RR: IdentityRoleRepository,
    IR::Error: std::fmt::Debug,
    CR::Error: std::fmt::Debug,
    PR::Error: std::fmt::Debug,
    RR::Error: std::fmt::Debug,
{
    let identity_id = IdentityId::from_uuid(DEMO_IDENTITY_UUID);

    let identity = Identity::new(identity_id);

    identity_repository
        .save(identity)
        .unwrap_or_else(|error| panic!("identity seed should succeed: {error:?}"));

    let credential_id = CredentialId::from_uuid(DEMO_CREDENTIAL_UUID);

    let credential = Credential::new(credential_id, identity_id, CredentialKind::Password);

    credential_repository
        .save(credential)
        .unwrap_or_else(|error| panic!("credential seed should succeed: {error:?}"));

    let hasher = Argon2PasswordHasher::new();

    let password = PasswordSecret::new("correct-password").expect("demo password should be valid");

    let password_hash = hasher
        .hash(&password)
        .expect("password hashing should succeed");

    let material = PasswordMaterial::new(credential_id, password_hash);

    password_material_repository
        .save(material)
        .unwrap_or_else(|error| panic!("password material seed should succeed: {error:?}"));

    let permission = Permission::new("user.read").expect("demo permission should be valid");

    let role = Role::new("admin", vec![permission]).expect("demo role should be valid");

    identity_role_repository
        .assign(identity_id, vec![role])
        .unwrap_or_else(|error| panic!("role assignment should succeed: {error:?}"));

    (credential_id, identity_id)
}

/// Seeds a demo local client application.
///
/// Returns client identifier.
pub fn seed_demo_client<CR>(client_repository: &mut CR) -> ClientId
where
    CR: ClientRepository,
    CR::Error: std::fmt::Debug,
{
    let client_id = ClientId::from_uuid(DEMO_CLIENT_UUID);

    let client = Client::new(client_id, client_id.to_string(), "LocalID Demo Application");

    client_repository
        .save(client)
        .unwrap_or_else(|error| panic!("client seed should succeed: {error:?}"));

    client_id
}

/// Seeds a demo OAuth client.
///
/// Returns internal OAuth client id and public OAuth client id.
pub fn seed_demo_oauth_client<R>(repository: &mut R) -> (OAuthClientId, String)
where
    R: OAuthClientRepository,
    R::Error: std::fmt::Debug,
{
    seed_oauth_client_with_id(
        repository,
        OAuthClientId::from_uuid(DEMO_OAUTH_CLIENT_UUID),
        "demo-client".to_string(),
    )
}

/// Seeds OAuth client with custom public id.
///
/// Returns internal OAuth client id and public OAuth client id.
pub fn seed_oauth_client<R>(repository: &mut R, public_client_id: String) -> (OAuthClientId, String)
where
    R: OAuthClientRepository,
    R::Error: std::fmt::Debug,
{
    let fallback_id = if public_client_id == "different-client" {
        OAuthClientId::from_uuid(DEMO_OAUTH_OTHER_CLIENT_UUID)
    } else {
        OAuthClientId::new()
    };

    seed_oauth_client_with_id(repository, fallback_id, public_client_id)
}

fn seed_oauth_client_with_id<R>(
    repository: &mut R,
    fallback_id: OAuthClientId,
    public_client_id: String,
) -> (OAuthClientId, String)
where
    R: OAuthClientRepository,
    R::Error: std::fmt::Debug,
{
    let existing = repository
        .find_by_client_id(&public_client_id)
        .unwrap_or_else(|error| panic!("oauth client lookup should succeed: {error:?}"));

    let client_secret = "demo-secret";

    let (internal_id, local_client_id) = match existing {
        Some(client) => (client.id(), client.local_client_id()),
        None => (fallback_id, ClientId::new()),
    };

    let client = OAuthClient::new(
        internal_id,
        local_client_id,
        public_client_id.clone(),
        "LocalID Demo Client",
        hash_secret(client_secret),
        vec!["http://localhost:3000/callback".to_string()],
    );

    repository
        .save(client)
        .expect("oauth client seed should succeed");

    (internal_id, public_client_id)
}
