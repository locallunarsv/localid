#![deny(missing_docs)]

//! Argon2 password hashing adapter for LocalID.

use argon2::{
    Argon2,
    password_hash::{
        PasswordHash as ParsedPasswordHash, PasswordHasher as _, PasswordVerifier as _, SaltString,
    },
};
use localid_password::{
    PasswordCredential, PasswordHash, PasswordHasher, PasswordSecret, PasswordVerifier,
};
use rand_core::OsRng;

/// Errors produced by the Argon2 password adapter.
#[derive(Debug)]
pub enum Argon2PasswordError {
    /// Argon2 could not hash the supplied password.
    HashingFailed(argon2::password_hash::Error),

    /// The stored password hash could not be parsed or verified.
    VerificationFailed(argon2::password_hash::Error),
}

impl std::fmt::Display for Argon2PasswordError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HashingFailed(_) => formatter.write_str("Argon2 password hashing failed"),
            Self::VerificationFailed(_) => {
                formatter.write_str("Argon2 password verification failed")
            }
        }
    }
}

impl std::error::Error for Argon2PasswordError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::HashingFailed(error) | Self::VerificationFailed(error) => Some(error),
        }
    }
}

impl PasswordVerifier for Argon2PasswordHasher {
    type Error = Argon2PasswordError;

    fn verify(
        &self,
        credential: &PasswordCredential,
        secret: &PasswordSecret,
    ) -> Result<bool, Self::Error> {
        PasswordHasher::verify(self, secret, credential.password_hash())
    }
}

/// Argon2 implementation of [`PasswordHasher`].
#[derive(Debug, Clone, Default)]
pub struct Argon2PasswordHasher {
    argon2: Argon2<'static>,
}

impl Argon2PasswordHasher {
    /// Creates an Argon2 password hasher using the crate defaults.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl PasswordHasher for Argon2PasswordHasher {
    type Error = Argon2PasswordError;

    fn hash(&self, secret: &PasswordSecret) -> Result<PasswordHash, Self::Error> {
        let salt = SaltString::generate(&mut OsRng);

        let encoded = self
            .argon2
            .hash_password(secret.as_str().as_bytes(), &salt)
            .map_err(Argon2PasswordError::HashingFailed)?
            .to_string();

        Ok(PasswordHash::new(encoded))
    }

    fn verify(&self, secret: &PasswordSecret, hash: &PasswordHash) -> Result<bool, Self::Error> {
        let parsed = ParsedPasswordHash::new(hash.as_str())
            .map_err(Argon2PasswordError::VerificationFailed)?;

        match self
            .argon2
            .verify_password(secret.as_str().as_bytes(), &parsed)
        {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(error) => Err(Argon2PasswordError::VerificationFailed(error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use localid_password::{PasswordHasher, PasswordSecret};

    use super::Argon2PasswordHasher;

    #[test]
    fn hashes_and_verifies_password() {
        let hasher = Argon2PasswordHasher::new();
        let secret =
            PasswordSecret::new("correct horse battery staple").expect("password should be valid");

        let hash = hasher
            .hash(&secret)
            .expect("password hashing should succeed");

        assert!(
            hasher
                .verify(&secret, &hash)
                .expect("password verification should succeed")
        );
    }

    #[test]
    fn rejects_incorrect_password() {
        let hasher = Argon2PasswordHasher::new();

        let correct = PasswordSecret::new("correct-password").expect("password should be valid");
        let incorrect =
            PasswordSecret::new("incorrect-password").expect("password should be valid");

        let hash = hasher
            .hash(&correct)
            .expect("password hashing should succeed");

        assert!(
            !hasher
                .verify(&incorrect, &hash)
                .expect("password verification should complete")
        );
    }

    #[test]
    fn generated_hashes_use_distinct_salts() {
        let hasher = Argon2PasswordHasher::new();
        let secret = PasswordSecret::new("same-password").expect("password should be valid");

        let first = hasher.hash(&secret).expect("first hash should succeed");
        let second = hasher.hash(&secret).expect("second hash should succeed");

        assert_ne!(first, second);
    }
    #[test]
    fn verifies_password_credential() {
        use localid_credential::CredentialId;
        use localid_password::{PasswordCredential, PasswordHasher, PasswordVerifier};

        let verifier = Argon2PasswordHasher::new();
        let secret = PasswordSecret::new("correct-password").expect("password should be valid");

        let hash =
            PasswordHasher::hash(&verifier, &secret).expect("password hashing should succeed");

        let credential = PasswordCredential::new(CredentialId::new(), hash);

        assert!(
            PasswordVerifier::verify(&verifier, &credential, &secret)
                .expect("verification should succeed")
        );
    }
}
