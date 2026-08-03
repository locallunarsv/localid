/// Login application use case.
///
/// Coordinates authentication flow between
/// external input and authentication domain.
use super::{AuthenticationPort, LoginCommand, TokenResponse, map_authentication_error};

use crate::ApplicationError;

/// Login application use case.
#[derive(Debug, Clone, Copy)]
pub struct LoginUseCase<A> {
    authentication_service: A,
}

impl<A> LoginUseCase<A>
where
    A: AuthenticationPort<Error = localid_authentication::AuthenticationError>,
{
    /// Creates a new login use case.
    #[must_use]
    pub const fn new(authentication_service: A) -> Self {
        Self {
            authentication_service,
        }
    }

    /// Executes login flow.
    pub fn execute(&mut self, command: LoginCommand) -> Result<TokenResponse, ApplicationError> {
        self.authentication_service
            .authenticate(command)
            .map(|result| TokenResponse::from_authentication_result(&result))
            .map_err(map_authentication_error)
    }
}
