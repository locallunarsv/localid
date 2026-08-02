use super::result::RefreshResult;

/// Refresh token service contract.
pub trait RefreshTokenService {
    /// Error returned during refresh.
    type Error;

    /// Rotates refresh token.
    fn refresh(&mut self, refresh_secret: &str) -> Result<RefreshResult, Self::Error>;
}
