//! Cryptographic key types.

use rand::rngs::OsRng;
use rsa::RsaPrivateKey;
use rsa::RsaPublicKey;

use std::path::Path;

use crate::KeyStorage;

use crate::CryptoError;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};

use rsa::traits::PublicKeyParts;

use crate::JsonWebKey;

/// Signing key identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyId(String);

impl KeyId {
    /// Creates a key identifier.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns key identifier value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// RSA signing key pair.
#[derive(Debug)]
pub struct KeyPair {
    kid: KeyId,
    private_key: RsaPrivateKey,
}

impl KeyPair {
    /// Generates a new RSA key pair.
    pub fn generate(kid: KeyId) -> Result<Self, CryptoError> {
        let mut rng = OsRng;

        let private_key =
            RsaPrivateKey::new(&mut rng, 2048).map_err(|_| CryptoError::KeyGenerationFailed)?;

        Ok(Self { kid, private_key })
    }

    /// Loads existing key or generates a new key.
    pub fn load_or_generate<S>(storage: &S, path: &Path, kid: KeyId) -> Result<Self, CryptoError>
    where
        S: KeyStorage,
    {
        match storage.load(path)? {
            Some(key) => Ok(key),

            None => {
                let key = Self::generate(kid)?;

                storage.save(path, &key)?;

                Ok(key)
            }
        }
    }

    /// Returns key identifier.
    #[must_use]
    pub fn kid(&self) -> &KeyId {
        &self.kid
    }

    /// Returns private RSA key.
    #[must_use]
    pub fn private_key(&self) -> &RsaPrivateKey {
        &self.private_key
    }

    /// Returns RSA public key.
    #[must_use]
    pub fn public_key(&self) -> RsaPublicKey {
        self.private_key.to_public_key()
    }

    /// Converts public key into JSON Web Key.
    #[must_use]
    pub fn to_jwk(&self) -> JsonWebKey {
        let public_key = self.public_key();

        let n = URL_SAFE_NO_PAD.encode(public_key.n().to_bytes_be());

        let e = URL_SAFE_NO_PAD.encode(public_key.e().to_bytes_be());

        JsonWebKey {
            kty: "RSA".to_string(),
            kid: self.kid.value().to_string(),
            use_: "sig".to_string(),
            alg: "RS256".to_string(),
            n,
            e,
        }
    }

    /// Creates key pair from existing RSA private key.
    #[must_use]
    pub fn from_private_key(kid: KeyId, private_key: RsaPrivateKey) -> Self {
        Self { kid, private_key }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_rsa_key_pair() {
        let key =
            KeyPair::generate(KeyId::new("localid-key-1")).expect("key generation should succeed");

        assert_eq!(key.kid().value(), "localid-key-1");
    }
}

#[test]
fn should_convert_public_key_to_jwk() {
    let key =
        KeyPair::generate(KeyId::new("localid-key-1")).expect("key generation should succeed");

    let jwk = key.to_jwk();

    assert_eq!(jwk.kty, "RSA");
    assert_eq!(jwk.kid, "localid-key-1");
    assert_eq!(jwk.alg, "RS256");
    assert!(!jwk.n.is_empty());
    assert!(!jwk.e.is_empty());
}

#[test]
fn should_reuse_existing_key() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("signing-key.pem");

    let storage = crate::FileKeyStorage::new();

    let first = KeyPair::load_or_generate(&storage, &path, KeyId::new("localid-key-1"))
        .expect("first key generation should succeed");

    let first_jwk = first.to_jwk();

    let second = KeyPair::load_or_generate(&storage, &path, KeyId::new("localid-key-1"))
        .expect("key loading should succeed");

    let second_jwk = second.to_jwk();

    assert_eq!(first_jwk.n, second_jwk.n);

    assert_eq!(first_jwk.e, second_jwk.e);
}
