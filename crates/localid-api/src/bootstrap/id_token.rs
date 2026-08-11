//! Bootstrap OpenID Connect ID token issuer.

use std::sync::Arc;

use localid_application::oauth::token_exchange::{IdTokenIssueError, IdTokenIssuer};

use localid_crypto::CryptoTokenSigner;

use localid_token::{IdTokenClaims, JwtEncoder};

/// Bootstrap ID token issuer.
pub struct BootstrapIdTokenIssuer {
    encoder: JwtEncoder<CryptoTokenSigner>,
}

impl BootstrapIdTokenIssuer {
    /// Creates a new bootstrap ID token issuer.
    #[must_use]
    pub fn new(key_pair: Arc<localid_crypto::KeyPair>) -> Self {
        Self {
            encoder: JwtEncoder::new(CryptoTokenSigner::new(key_pair), "localid-key-1"),
        }
    }
}

impl IdTokenIssuer for BootstrapIdTokenIssuer {
    fn issue(&self, claims: IdTokenClaims) -> Result<String, IdTokenIssueError> {
        self.encoder
            .encode(&claims)
            .map_err(|_| IdTokenIssueError::SigningFailed)
    }
}
