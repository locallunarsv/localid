use super::super::{AuthenticationPort, LoginCommand};

use localid_authentication::{
    AuthenticatePasswordRequest, AuthenticateResult, AuthenticationError,
    PasswordAuthenticationService,
};

/// Adapter for password authentication service.
pub struct PasswordAuthenticationAdapter<S> {
    service: S,
}

impl<S> PasswordAuthenticationAdapter<S> {
    /// Creates a new password authentication adapter.
    #[must_use]
    pub const fn new(service: S) -> Self {
        Self { service }
    }
}

impl<S> AuthenticationPort for PasswordAuthenticationAdapter<S>
where
    S: PasswordAuthenticationService,
{
    type Error = AuthenticationError;

    fn authenticate(&mut self, command: LoginCommand) -> Result<AuthenticateResult, Self::Error> {
        let request = AuthenticatePasswordRequest::new(
            command.client_id(),
            command.credential_id(),
            command.password().clone(),
        );

        self.service.authenticate_password(request)
    }
}
