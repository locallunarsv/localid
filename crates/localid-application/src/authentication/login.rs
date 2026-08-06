use super::{AuthenticationPort, LoginCommand, TokenResponse, map_authentication_error};

use crate::{
    ApplicationError,
    client::{ClientPort, FindClientQuery, GetClientUseCase},
};

/// Login application use case.
///
/// Coordinates client validation and authentication flow.
#[derive(Debug)]
pub struct LoginUseCase<A, C> {
    authentication_service: A,
    client_use_case: GetClientUseCase<C>,
}

impl<A, C> LoginUseCase<A, C>
where
    A: AuthenticationPort<Error = localid_authentication::AuthenticationError>,
    C: ClientPort,
{
    /// Creates a new login use case.
    #[must_use]
    pub const fn new(authentication_service: A, client_use_case: GetClientUseCase<C>) -> Self {
        Self {
            authentication_service,
            client_use_case,
        }
    }

    /// Executes login flow.
    pub fn execute(&mut self, command: LoginCommand) -> Result<TokenResponse, ApplicationError> {
        self.client_use_case
            .execute(FindClientQuery::new(command.client_id().to_string()))
            .map_err(|_| ApplicationError::AuthenticationFailed)?;

        self.authentication_service
            .authenticate(command)
            .map(|result| TokenResponse::from_authentication_result(&result))
            .map_err(map_authentication_error)
    }
}
