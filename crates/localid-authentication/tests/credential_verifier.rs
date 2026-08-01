use localid_authentication::{AuthenticationEvidence, CredentialVerifier};
use localid_credential::{Credential, CredentialId, CredentialKind};
use localid_identity::IdentityId;
use localid_password::PasswordSecret;

struct AcceptingVerifier;

impl CredentialVerifier for AcceptingVerifier {
    type Error = ();

    fn verify(
        &self,
        _credential: &Credential,
        _evidence: &AuthenticationEvidence,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

struct RejectingVerifier;

impl CredentialVerifier for RejectingVerifier {
    type Error = ();

    fn verify(
        &self,
        _credential: &Credential,
        _evidence: &AuthenticationEvidence,
    ) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

fn credential() -> Credential {
    Credential::new(
        CredentialId::new(),
        IdentityId::new(),
        CredentialKind::Password,
    )
}

#[test]
fn verifier_can_accept_authentication_evidence() {
    let verifier = AcceptingVerifier;
    let evidence = AuthenticationEvidence::password(
        PasswordSecret::new("test-password").expect("test password should be valid"),
    );
    let credential = credential();

    let verified = verifier
        .verify(&credential, &evidence)
        .expect("verification should complete");

    assert!(verified);
}

#[test]
fn verifier_can_reject_authentication_evidence() {
    let verifier = RejectingVerifier;
    let evidence = AuthenticationEvidence::password(
        PasswordSecret::new("test-password").expect("test password should be valid"),
    );
    let credential = credential();

    let verified = verifier
        .verify(&credential, &evidence)
        .expect("verification should complete");

    assert!(!verified);
}
