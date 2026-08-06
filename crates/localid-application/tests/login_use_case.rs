use chrono::{TimeDelta, Utc};

use localid_application::{ApplicationError, AuthenticationPort, LoginCommand, LoginUseCase};

use localid_authentication::{AuthenticateResult, AuthenticationError};

use localid_client::ClientId;
use localid_credential::CredentialId;
use localid_identity::IdentityId;
use localid_password::PasswordSecret;
use localid_refresh_token::RefreshToken;
use localid_refresh_token_random::IssuedRefreshToken;
use localid_session::{Session, SessionId};
use localid_token::Token;
use localid_token_random::IssuedToken;

enum FakeAuthenticationResponse {
    Success(Box<AuthenticateResult>),
    Failure(AuthenticationError),
}

struct FakeAuthenticationService {
    response: FakeAuthenticationResponse,
}

impl AuthenticationPort for FakeAuthenticationService {
    type Error = AuthenticationError;

    fn authenticate(&mut self, _command: LoginCommand) -> Result<AuthenticateResult, Self::Error> {
        match std::mem::replace(
            &mut self.response,
            FakeAuthenticationResponse::Failure(AuthenticationError::IdentityUnavailable),
        ) {
            FakeAuthenticationResponse::Success(result) => Ok(*result),
            FakeAuthenticationResponse::Failure(error) => Err(error),
        }
    }
}

fn authenticated_result() -> AuthenticateResult {
    let identity_id = IdentityId::new();
    let client_id = ClientId::new();

    let created_at = Utc::now();

    let session = Session::new(
        SessionId::new(),
        identity_id,
        client_id,
        created_at,
        created_at + TimeDelta::hours(1),
    )
    .expect("session should be valid");

    let token = IssuedToken::new(
        Token::new(
            localid_token::TokenId::new(),
            session.id(),
            "token-hash".to_owned(),
            session.created_at(),
            session.expires_at(),
        )
        .expect("token should be valid"),
        "token-secret".to_owned(),
    );

    let refresh_token = IssuedRefreshToken::new(
        RefreshToken::new(
            localid_refresh_token::RefreshTokenId::new(),
            session.id(),
            "refresh-hash".to_owned(),
            session.created_at(),
            session.expires_at(),
        )
        .expect("refresh token should be valid"),
        "refresh-secret".to_owned(),
    );

    AuthenticateResult::new(session, token, refresh_token)
}

#[test]
fn authentication_failure_maps_to_application_error() {
    let service = FakeAuthenticationService {
        response: FakeAuthenticationResponse::Failure(AuthenticationError::InvalidPassword),
    };

    let mut use_case = LoginUseCase::new(service);

    let command = LoginCommand::new(
        ClientId::new(),
        CredentialId::new(),
        PasswordSecret::new("wrong-password").expect("password should be valid"),
    );

    let result = use_case.execute(command);

    assert!(matches!(
        result,
        Err(ApplicationError::AuthenticationFailed)
    ));
}

#[test]
fn successful_authentication_returns_response() {
    let service = FakeAuthenticationService {
        response: FakeAuthenticationResponse::Success(Box::new(authenticated_result())),
    };

    let mut use_case = LoginUseCase::new(service);

    let command = LoginCommand::new(
        ClientId::new(),
        CredentialId::new(),
        PasswordSecret::new("correct-password").expect("password should be valid"),
    );

    let result = use_case
        .execute(command)
        .expect("authentication should succeed");

    assert!(!result.access_token().is_empty());
    assert!(!result.refresh_token().is_empty());
}
