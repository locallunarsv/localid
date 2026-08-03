use super::LoginCommand;

use localid_authentication::AuthenticateResult;

/// Authentication port.
///
/// Defines authentication capability required
/// by application use cases.
pub trait AuthenticationPort {
    /// Error returned by authentication.
    type Error;

    /// Authentication result.
    fn authenticate(&mut self, command: LoginCommand) -> Result<AuthenticateResult, Self::Error>;
}
