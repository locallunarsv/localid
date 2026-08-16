use localid_client::{Client, ClientId};
use localid_credential::{Credential, CredentialId, CredentialKind};
use localid_identity::{Identity, IdentityId};
use localid_oauth_client::{OAuthClient, OAuthClientId, OAuthClientRepository};
use localid_password::{PasswordHasher, PasswordMaterial, PasswordSecret};
use localid_password_argon2::Argon2PasswordHasher;
use localid_permission::Permission;
use localid_repository::{
    ClientRepository, CredentialRepository, IdentityRepository, PasswordMaterialRepository,
};
use localid_repository_memory::MemoryIdentityRoleRepository;
use localid_role::Role;

use localid_crypto::hash_secret;

/// Seeds a demo password identity.
///
/// Returns credential identifier and identity identifier.
pub fn seed_demo_identity<IR, CR, PR>(
    identity_repository: &mut IR,
    credential_repository: &mut CR,
    password_material_repository: &mut PR,
    identity_role_repository: &mut MemoryIdentityRoleRepository,
) -> (CredentialId, IdentityId)
where
    IR: IdentityRepository,
    CR: CredentialRepository,
    PR: PasswordMaterialRepository,
{
    let identity_id = IdentityId::new();

    let identity = Identity::new(identity_id);

    identity_repository
        .save(identity)
        .unwrap_or_else(|_| panic!("identity seed should succeed"));

    let credential_id = CredentialId::new();

    let credential = Credential::new(credential_id, identity_id, CredentialKind::Password);

    credential_repository
        .save(credential)
        .unwrap_or_else(|_| panic!("credential seed should succeed"));

    let hasher = Argon2PasswordHasher::new();

    let password = PasswordSecret::new("correct-password").expect("demo password should be valid");

    let password_hash = hasher
        .hash(&password)
        .expect("password hashing should succeed");

    let material = PasswordMaterial::new(credential_id, password_hash);

    password_material_repository
        .save(material)
        .unwrap_or_else(|_| panic!("password material seed should succeed"));

    let permission = Permission::new("user.read").expect("demo permission should be valid");

    let role = Role::new("admin", vec![permission]).expect("demo role should be valid");

    identity_role_repository.assign(identity_id, vec![role]);

    (credential_id, identity_id)
}

/// Seeds a demo local client application.
///
/// Returns client identifier.
pub fn seed_demo_client<CR>(client_repository: &mut CR) -> ClientId
where
    CR: ClientRepository,
{
    let client_id = ClientId::new();

    let client = Client::new(client_id, client_id.to_string(), "LocalID Demo Application");

    client_repository
        .save(client)
        .unwrap_or_else(|_| panic!("client seed should succeed"));

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
    seed_oauth_client(repository, "demo-client".to_string())
}

/// Seeds OAuth client with custom public id.
///
/// Returns internal OAuth client id and public OAuth client id.
pub fn seed_oauth_client<R>(repository: &mut R, public_client_id: String) -> (OAuthClientId, String)
where
    R: OAuthClientRepository,
    R::Error: std::fmt::Debug,
{
    let local_client_id = ClientId::new();

    let client_secret = "demo-secret";

    let client = OAuthClient::new(
        OAuthClientId::new(),
        local_client_id,
        public_client_id.clone(),
        "LocalID Demo Client",
        hash_secret(client_secret),
        vec!["http://localhost:3000/callback".to_string()],
    );

    let internal_id = client.id();

    repository
        .save(client)
        .expect("oauth client seed should succeed");

    (internal_id, public_client_id)
}
