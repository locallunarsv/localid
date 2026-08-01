use super::{PasswordCredential, PasswordSecret};

/// Verifies a supplied password against stored password Credential material.
pub trait PasswordVerifier {
    /// Error produced by the concrete verification implementation.
    type Error;

    /// Verifies whether the supplied password matches the stored Credential.
    ///
    /// Returns `true` when the password is valid and `false` when it does not
    /// match.
    ///
    /// # Errors
    ///
    /// Returns the concrete verifier error when verification cannot be
    /// completed.
    fn verify(
        &self,
        credential: &PasswordCredential,
        secret: &PasswordSecret,
    ) -> Result<bool, Self::Error>;
}
