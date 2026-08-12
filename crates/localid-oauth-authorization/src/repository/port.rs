use super::super::{AuthorizationCode, AuthorizationCodeId};

/// Repository port for authorization codes.
pub trait AuthorizationCodeRepository {
    /// Repository error.
    type Error;

    /// Saves authorization code.
    fn save(&mut self, code: AuthorizationCode) -> Result<(), Self::Error>;

    /// Finds authorization code by id.
    fn find_by_id(&self, id: AuthorizationCodeId)
    -> Result<Option<AuthorizationCode>, Self::Error>;

    /// Finds authorization code by hashed secret.
    fn find_by_hash(&self, hash: &str) -> Result<Option<AuthorizationCode>, Self::Error>;
}
