use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha256};

/// Random authorization code generator.
#[derive(Debug, Default, Clone, Copy)]
pub struct RandomAuthorizationCodeGenerator;

impl RandomAuthorizationCodeGenerator {
    /// Creates a new generator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Generates a random authorization code.
    #[must_use]
    pub fn generate(&self) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect()
    }

    /// Hashes authorization code.
    #[must_use]
    pub fn hash(&self, value: &str) -> String {
        let mut hasher = Sha256::new();

        hasher.update(value.as_bytes());

        hex::encode(hasher.finalize())
    }
}
