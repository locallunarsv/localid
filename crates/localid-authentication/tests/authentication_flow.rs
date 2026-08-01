use chrono::{TimeDelta, TimeZone, Utc};
use localid_authentication::{
    AuthenticateRequest, AuthenticationEvidence, AuthenticationService, CredentialVerifier,
    DefaultAuthenticationService, SessionFactory,
};
use localid_credential::{Credential, CredentialId, CredentialKind};
use localid_identity::{Identity, IdentityId};
use localid_repository::{CredentialRepository, IdentityRepository, SessionRepository};
use localid_session::{Session, SessionId};
use localid_storage_memory::MemoryStorage;

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
fn successful_authentication_creates_and_stores_session() {
    let mut storage = MemoryStorage::new();

    let identity = Identity::new(IdentityId::new());
    let identity_id = identity.id();

    IdentityRepository::save(&mut storage, identity).expect("Identity should be stored");

    let credential = Credential::new(CredentialId::new(), identity_id, CredentialKind::Password);
    let credential_id = credential.id();

    CredentialRepository::save(&mut storage, credential).expect("Credential should be stored");

    let mut authentication = DefaultAuthenticationService::new(
        storage.clone(),
        storage.clone(),
        storage.clone(),
        AcceptingVerifier,
        FixedSessionFactory,
    );

    let request = AuthenticateRequest::new(credential_id, AuthenticationEvidence);

    let result = authentication
        .authenticate(request)
        .expect("authentication should succeed");

    let session = result.session();

    assert_eq!(session.identity_id(), identity_id);
    assert!(session.is_active());

    let stored_session = SessionRepository::find_by_id(&storage, session.id())
        .expect("Session lookup should succeed")
        .expect("authenticated Session should be stored");

    assert_eq!(stored_session, *session);
}
