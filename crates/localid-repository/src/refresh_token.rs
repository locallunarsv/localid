use localid_refresh_token::{RefreshToken, RefreshTokenId};

/// Repository contract for Refresh Token persistence.
pub trait RefreshTokenRepository {
    /// Repository-specific error.
    type Error;

    /// Finds a Refresh Token by identifier.
    fn find_by_id(&self, id: RefreshTokenId) -> Result<Option<RefreshToken>, Self::Error>;

    /// Finds a Refresh Token by stored secret hash.
    fn find_by_secret_hash(&self, secret_hash: &str) -> Result<Option<RefreshToken>, Self::Error>;

    /// Persists a Refresh Token.
    fn save(&mut self, token: RefreshToken) -> Result<(), Self::Error>;
}
