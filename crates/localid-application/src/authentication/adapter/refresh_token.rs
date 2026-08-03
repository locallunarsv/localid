use super::super::RefreshTokenPort;

use localid_authentication::{AuthenticationError, RefreshResult, RefreshTokenService};

/// Adapter for refresh token authentication service.
pub struct RefreshTokenAdapter<S> {
    service: S,
}

impl<S> RefreshTokenAdapter<S> {
    /// Creates a new refresh token adapter.
    #[must_use]
    pub const fn new(service: S) -> Self {
        Self { service }
    }
}

impl<S> RefreshTokenPort for RefreshTokenAdapter<S>
where
    S: RefreshTokenService<Error = AuthenticationError>,
{
    type Error = AuthenticationError;

    fn refresh(&mut self, refresh_token: &str) -> Result<RefreshResult, Self::Error> {
        self.service.refresh(refresh_token)
    }
}
