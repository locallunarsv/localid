use super::{PasswordHash, PasswordSecret};

/// Hashes and verifies passwords.
pub trait PasswordHasher {
    /// Error produced by the hasher.
    type Error;

    /// Produces a password hash.
    fn hash(&self, secret: &PasswordSecret) -> Result<PasswordHash, Self::Error>;

    /// Verifies a password against a stored hash.
    fn verify(&self, secret: &PasswordSecret, hash: &PasswordHash) -> Result<bool, Self::Error>;
}
