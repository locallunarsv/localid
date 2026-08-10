//! Token signing adapter.

use std::sync::Arc;

use localid_token::{TokenSigner, TokenSigningError};

use crate::KeyPair;

/// RSA token signer implementation.
#[derive(Clone)]
pub struct CryptoTokenSigner {
    key_pair: Arc<KeyPair>,
}

impl CryptoTokenSigner {
    /// Creates RSA token signer.
    #[must_use]
    pub fn new(key_pair: Arc<KeyPair>) -> Self {
        Self { key_pair }
    }
}

impl TokenSigner for CryptoTokenSigner {
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>, TokenSigningError> {
        self.key_pair
            .sign_sha256(payload)
            .map_err(|_| TokenSigningError::SigningFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sign_payload_using_rsa_key() {
        let key_pair = KeyPair::generate(crate::KeyId::new("localid-key-1"))
            .expect("key generation should succeed");

        let signer = CryptoTokenSigner::new(Arc::new(key_pair));

        let signature = signer
            .sign(b"header.payload")
            .expect("signing should succeed");

        assert!(!signature.is_empty());
    }
}
