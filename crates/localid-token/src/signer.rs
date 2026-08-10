//! Token signing abstraction.

/// Error returned by token signing.
#[derive(Debug)]
pub enum TokenSigningError {
    /// Token signing operation failed.
    SigningFailed,
}

/// Signs JWT tokens.
pub trait TokenSigner {
    /// Signs arbitrary payload bytes.
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>, TokenSigningError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeSigner;

    impl TokenSigner for FakeSigner {
        fn sign(&self, _payload: &[u8]) -> Result<Vec<u8>, TokenSigningError> {
            Ok(b"dummy-signature".to_vec())
        }
    }

    #[test]
    fn should_sign_payload() {
        let signer = FakeSigner;

        let signature = signer.sign(b"header.payload").unwrap();

        assert_eq!(signature, b"dummy-signature");
    }
}
