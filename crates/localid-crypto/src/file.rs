//! File based key storage.

use std::fs;

use crate::KeyStorage;

use std::path::Path;

use rsa::{
    RsaPrivateKey,
    pkcs8::{DecodePrivateKey, EncodePrivateKey},
};

use crate::{CryptoError, KeyId, KeyPair};

/// File based key storage.
pub struct FileKeyStorage;

impl FileKeyStorage {
    /// Creates file key storage.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for FileKeyStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyStorage for FileKeyStorage {
    fn load(&self, path: &Path) -> Result<Option<KeyPair>, CryptoError> {
        if !path.exists() {
            return Ok(None);
        }

        let pem = fs::read_to_string(path).map_err(|_| CryptoError::StorageFailure)?;

        let private_key =
            RsaPrivateKey::from_pkcs8_pem(&pem).map_err(|_| CryptoError::SerializationFailed)?;

        Ok(Some(KeyPair::from_private_key(
            KeyId::new("localid-key-1"),
            private_key,
        )))
    }

    fn save(&self, path: &Path, key: &KeyPair) -> Result<(), CryptoError> {
        let pem = key
            .private_key()
            .to_pkcs8_pem(Default::default())
            .map_err(|_| CryptoError::SerializationFailed)?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|_| CryptoError::StorageFailure)?;
        }

        fs::write(path, pem.as_bytes()).map_err(|_| CryptoError::StorageFailure)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    use crate::{KeyId, KeyPair, KeyStorage};

    #[test]
    fn should_save_and_load_key() {
        let dir = tempdir().unwrap();

        let path = dir.path().join("signing-key.pem");

        let key =
            KeyPair::generate(KeyId::new("localid-key-1")).expect("key generation should succeed");

        let storage = FileKeyStorage::new();

        storage.save(&path, &key).expect("key save should succeed");

        let loaded = storage
            .load(&path)
            .expect("key load should succeed")
            .expect("key should exist");

        assert_eq!(loaded.kid().value(), "localid-key-1");
    }
}
