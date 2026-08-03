use localid_authentication::RefreshResult;

use super::map_authentication_error;
use crate::{ApplicationError, TokenResponse};

/// Port for refresh token authentication.
pub trait RefreshTokenPort {
    /// Error returned by refresh operation.
    type Error;

    /// Refreshes authentication tokens.
    fn refresh(&mut self, refresh_token: &str) -> Result<RefreshResult, Self::Error>;
}

/// Refresh token application use case.
pub struct RefreshTokenUseCase<A> {
    refresh_service: A,
}

impl<A> RefreshTokenUseCase<A>
where
    A: RefreshTokenPort<Error = localid_authentication::AuthenticationError>,
{
    /// Creates a new refresh token use case.
    #[must_use]
    pub const fn new(refresh_service: A) -> Self {
        Self { refresh_service }
    }

    /// Executes refresh token flow.
    pub fn execute(&mut self, refresh_token: &str) -> Result<TokenResponse, ApplicationError> {
        self.refresh_service
            .refresh(refresh_token)
            .map(|result| TokenResponse::from_refresh_result(&result))
            .map_err(map_authentication_error)
    }
}
