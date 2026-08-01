use super::AuthenticatePasswordRequest;
use crate::{AuthenticateResult, AuthenticationError};

/// Authenticates subjects using password Credentials.
pub trait PasswordAuthenticationService {
    /// Attempts password authentication.
    ///
    /// # Errors
    ///
    /// Returns an [`AuthenticationError`] when authentication cannot be
    /// completed successfully.
    fn authenticate_password(
        &mut self,
        request: AuthenticatePasswordRequest,
    ) -> Result<AuthenticateResult, AuthenticationError>;
}
