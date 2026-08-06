use chrono::{Duration, Utc};

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
            .find_client(&command.client_id().to_string())
            .map_err(|_| ApplicationError::InternalFailure)?
            .ok_or(ApplicationError::InternalFailure)?;

        if !client.state().is_active() {
            return Err(ApplicationError::InternalFailure);
        }

        if !client.allows_redirect_uri(command.redirect_uri()) {
            return Err(ApplicationError::InternalFailure);
        }

        let now = Utc::now();

        let code = AuthorizationCode::new(
            AuthorizationCodeId::new(),
            client.id(),
            command.identity_id(),
            "authorization-code-hash",
            command.redirect_uri(),
            now,
            now + Duration::minutes(10),
        )
        .map_err(|_| ApplicationError::InternalFailure)?;

        self.port
            .save_code(code.clone())
            .map_err(|_| ApplicationError::InternalFailure)?;

        Ok(AuthorizationResult::new(code))
    }
}
