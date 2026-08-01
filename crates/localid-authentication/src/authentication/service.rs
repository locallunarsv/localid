use super::{AuthenticateRequest, AuthenticateResult, AuthenticationError};

/// Authenticates subjects using LocalID domain models.
pub trait AuthenticationService {
    /// Attempts to authenticate a subject.
    ///
    /// # Errors
    ///
    /// Returns an [`AuthenticationError`] when authentication cannot be
    /// completed successfully.
    fn authenticate(
        &mut self,
        request: AuthenticateRequest,
    ) -> Result<AuthenticateResult, AuthenticationError>;
}
