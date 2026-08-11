//! Token verification adapter.

use std::sync::Arc;

use localid_token::{TokenSigningError, TokenVerifier};

use crate::KeyPair;

/// RSA token verifier implementation.
#[derive(Clone)]
pub struct CryptoTokenVerifier {
    key_pair: Arc<KeyPair>,
}

impl CryptoTokenVerifier {
    /// Creates RSA token verifier.
    #[must_use]
    pub fn new(key_pair: Arc<KeyPair>) -> Self {
        Self { key_pair }
    }
}

impl TokenVerifier for CryptoTokenVerifier {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<bool, TokenSigningError> {
        self.key_pair
            .verify_sha256(payload, signature)
            .map_err(|_| TokenSigningError::SigningFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;

    use crate::{KeyId, KeyPair};

    use localid_token::{TokenSigner, TokenVerifier};

    #[test]
    fn should_verify_payload_using_rsa_key() {
        let key_pair =
            KeyPair::generate(KeyId::new("localid-key-1")).expect("key generation should succeed");

        let key_pair = Arc::new(key_pair);

        let signer = crate::CryptoTokenSigner::new(key_pair.clone());

        let verifier = CryptoTokenVerifier::new(key_pair);

        let payload = b"header.payload";

        let signature = signer.sign(payload).expect("sign should succeed");

        let valid = verifier
            .verify(payload, &signature)
            .expect("verify should succeed");

        assert!(valid);
    }
}
