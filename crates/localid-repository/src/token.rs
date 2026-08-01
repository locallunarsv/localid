use localid_token::{Token, TokenId};

/// Repository contract for Token persistence.
pub trait TokenRepository {
    /// Repository-specific error.
    type Error;

    /// Finds a Token by identifier.
    ///
    /// # Errors
    ///
    /// Returns repository error when lookup cannot be completed.
    fn find_by_id(&self, id: TokenId) -> Result<Option<Token>, Self::Error>;

    /// Finds a Token by its stored secret hash.
    ///
    /// # Errors
    ///
    /// Returns repository error when lookup cannot be completed.
    fn find_by_secret_hash(&self, secret_hash: &str) -> Result<Option<Token>, Self::Error>;

    /// Persists a Token.
    ///
    /// # Errors
    ///
    /// Returns repository error when persistence cannot be completed.
    fn save(&mut self, token: Token) -> Result<(), Self::Error>;
}
