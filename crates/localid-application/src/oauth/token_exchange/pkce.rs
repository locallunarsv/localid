use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};

/// Verifies PKCE code verifier against stored code challenge.
#[must_use]
pub fn verify(code_verifier: &str, code_challenge: &str) -> bool {
    let mut hasher = Sha256::new();

    hasher.update(code_verifier.as_bytes());

    let hash = hasher.finalize();

    let calculated = URL_SAFE_NO_PAD.encode(hash);

    calculated == code_challenge
}
