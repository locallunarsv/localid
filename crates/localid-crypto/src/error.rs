//! Cryptographic errors.

/// Crypto operation errors.
#[derive(Debug, PartialEq, Eq)]
pub enum CryptoError {
    /// Key generation failed.
    KeyGenerationFailed,

    /// Key serialization failed.
    SerializationFailed,

    /// Key storage failure.
    StorageFailure,

    /// Signing operation failed.
    SigningFailed,
}
