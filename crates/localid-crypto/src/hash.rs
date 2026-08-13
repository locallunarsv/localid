use sha2::{Digest, Sha256};

/// Hashes secret value using SHA-256.
#[must_use]
pub fn hash_secret(secret: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(secret.as_bytes());

    hex::encode(hasher.finalize())
}
