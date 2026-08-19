//! Test bootstrap helpers.

use localid_client::ClientId;
use localid_credential::CredentialId;
use localid_identity::IdentityId;
use localid_oauth_client::{OAuthClientId, OAuthClientRepository};
use localid_repository::{
    ClientRepository, CredentialRepository, IdentityRepository, IdentityRoleRepository,
    PasswordMaterialRepository,
};

use super::seed::{
    seed_demo_client, seed_demo_identity, seed_demo_oauth_client, seed_oauth_client,
};

/// Test bootstrap seeded identifiers.
pub struct TestSeedContext {
    /// Seeded credential identifier.
    pub credential_id: CredentialId,

    /// Seeded identity identifier.
    pub identity_id: IdentityId,

    /// Seeded local client identifier.
    pub client_id: ClientId,

    /// Seeded OAuth internal identifier.
    pub oauth_client_id: OAuthClientId,

    /// Seeded OAuth public identifier.
    pub oauth_client_public_id: String,

    /// Seeded OAuth secret.
    pub oauth_client_secret: String,

    /// Seeded second OAuth public identifier.
    pub oauth_client_other_public_id: String,
}

/// Seeds all data required by API integration tests.
pub fn seed_test_environment<IR, CR, PR, RR, CL, OCR>(
    identity_repository: &mut IR,
    credential_repository: &mut CR,
    password_material_repository: &mut PR,
    identity_role_repository: &mut RR,
    client_repository: &mut CL,
    oauth_client_repository: &mut OCR,
) -> TestSeedContext
where
    IR: IdentityRepository,
    CR: CredentialRepository,
    PR: PasswordMaterialRepository,
    RR: IdentityRoleRepository,
    CL: ClientRepository,
    OCR: OAuthClientRepository,

    IR::Error: std::fmt::Debug,
    CR::Error: std::fmt::Debug,
    PR::Error: std::fmt::Debug,
    RR::Error: std::fmt::Debug,
    CL::Error: std::fmt::Debug,
    OCR::Error: std::fmt::Debug,
{
    let (credential_id, identity_id) = seed_demo_identity(
        identity_repository,
        credential_repository,
        password_material_repository,
        identity_role_repository,
    );

    let client_id = seed_demo_client(client_repository);

    let (oauth_client_id, oauth_client_public_id) = seed_demo_oauth_client(oauth_client_repository);
    let (_, oauth_client_other_public_id) =
        seed_oauth_client(oauth_client_repository, "different-client".to_string());

    TestSeedContext {
        credential_id,
        identity_id,
        client_id,
        oauth_client_id,
        oauth_client_public_id,
        oauth_client_secret: "demo-secret".to_string(),
        oauth_client_other_public_id,
    }
}
