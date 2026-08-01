use localid_authentication::{
    AuthenticateRequest, AuthenticateResult, AuthenticationError, AuthenticationEvidence,
    AuthenticationService,
};
use localid_credential::CredentialId;
use localid_identity::IdentityId;
use localid_session::Session;
use localid_session::SessionId;

use chrono::{TimeDelta, TimeZone, Utc};

struct StubAuthenticationService;

impl AuthenticationService for StubAuthenticationService {
    fn authenticate(
        &mut self,
        _request: AuthenticateRequest,
    ) -> Result<AuthenticateResult, AuthenticationError> {
        let created_at = Utc
            .with_ymd_and_hms(2026, 8, 2, 0, 0, 0)
            .single()
            .expect("valid timestamp");

        let session = Session::new(
            SessionId::new(),
            IdentityId::new(),
            created_at,
            created_at + TimeDelta::hours(1),
        )
        .expect("valid session");

        Ok(AuthenticateResult::new(session))
    }
}

#[test]
fn authentication_service_returns_result() {
    let mut service = StubAuthenticationService;

    let request = AuthenticateRequest::new(CredentialId::new(), AuthenticationEvidence);

    let result = service.authenticate(request);

    assert!(result.is_ok());
}
