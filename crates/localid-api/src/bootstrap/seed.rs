// crates/localid-api/src/bootstrap/seed.rs

use localid_credential::{Credential, CredentialId, CredentialKind};
use localid_identity::{Identity, IdentityId};
use localid_password::{PasswordHasher, PasswordMaterial, PasswordSecret};
use localid_password_argon2::Argon2PasswordHasher;
use localid_repository::{CredentialRepository, IdentityRepository, PasswordMaterialRepository};

/// Seeds a demo password identity.
///
/// Returns the generated Credential identifier.
pub fn seed_demo_identity<IR, CR, PR>(
    identity_repository: &mut IR,
    credential_repository: &mut CR,
    password_material_repository: &mut PR,
) -> CredentialId
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

    credential_id
}
