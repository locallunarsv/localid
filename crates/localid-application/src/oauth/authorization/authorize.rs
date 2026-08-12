use chrono::{Duration, Utc};

use localid_authorization_code_random::RandomAuthorizationCodeGenerator;
use localid_oauth_authorization::{AuthorizationCode, AuthorizationCodeId};

use crate::ApplicationError;

use super::{AuthorizationPort, AuthorizationResult, AuthorizeCommand};

/// OAuth authorization use case.
pub struct AuthorizeUseCase<P> {
    port: P,
}

impl<P> AuthorizeUseCase<P> {
    /// Creates a new authorization use case.
    #[must_use]
    pub const fn new(port: P) -> Self {
        Self { port }
    }
}

impl<P> AuthorizeUseCase<P>
where
    P: AuthorizationPort,
{
    /// Executes OAuth authorization flow.
    pub fn execute(
        &mut self,
        command: AuthorizeCommand,
    ) -> Result<AuthorizationResult, ApplicationError> {
        let client = self
            .port
            .find_client(&command.client_id())
            .map_err(|_| ApplicationError::InternalFailure)?
            .ok_or(ApplicationError::InternalFailure)?;

        if !client.state().is_active() {
            return Err(ApplicationError::InternalFailure);
        }

        if !client.allows_redirect_uri(command.redirect_uri()) {
            return Err(ApplicationError::InternalFailure);
        }

        let generator = RandomAuthorizationCodeGenerator::new();

        let authorization_code = generator.generate();

        let authorization_code_hash = generator.hash(&authorization_code);

        let now = Utc::now();

        let code = AuthorizationCode::new_with_nonce_and_pkce(
            AuthorizationCodeId::new(),
            client.id(),
            command.identity_id(),
            authorization_code_hash,
            command.redirect_uri(),
            command.nonce().map(ToOwned::to_owned),
            command.scope().to_vec(),
            command.request_state().map(ToOwned::to_owned),
            command.code_challenge().map(ToOwned::to_owned),
            command.code_challenge_method().cloned(),
            now,
            now + Duration::minutes(10),
        )
        .map_err(|_| ApplicationError::InternalFailure)?;

        self.port
            .save_code(code.clone())
            .map_err(|_| ApplicationError::InternalFailure)?;

        Ok(AuthorizationResult::new(code, authorization_code))
    }
}
