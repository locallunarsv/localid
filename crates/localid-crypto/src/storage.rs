//! Signing key storage.

use std::path::Path;

use crate::{CryptoError, KeyPair};

/// Key storage abstraction.
pub trait KeyStorage {
    /// Loads signing key.
    fn load(&self, path: &Path) -> Result<Option<KeyPair>, CryptoError>;

    /// Saves signing key.
    fn save(&self, path: &Path, key: &KeyPair) -> Result<(), CryptoError>;
}
