use localid_crypto::hash_secret;
use rand::{distributions::Alphanumeric, Rng};

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
        hash_secret(value)
    }
}
