//! JWT verification utilities.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

use serde::de::DeserializeOwned;

use crate::TokenSigningError;

/// Verifies JWT signature.
pub trait TokenVerifier {
    /// Verifies signature.
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<bool, TokenSigningError>;
}

/// JWT verifier.
pub struct JwtVerifier<V> {
    verifier: V,
}

impl<V> JwtVerifier<V>
where
    V: TokenVerifier,
{
    /// Creates JWT verifier.
    #[must_use]
    pub fn new(verifier: V) -> Self {
        Self { verifier }
    }

    /// Verifies JWT and returns claims.
    pub fn verify<T>(&self, token: &str) -> Result<T, TokenSigningError>
    where
        T: DeserializeOwned,
    {
        let parts: Vec<&str> = token.split('.').collect();

        if parts.len() != 3 {
            return Err(TokenSigningError::SigningFailed);
        }

        let signing_input = format!("{}.{}", parts[0], parts[1]);

        let signature = URL_SAFE_NO_PAD
            .decode(parts[2])
            .map_err(|_| TokenSigningError::SigningFailed)?;

        let valid = self.verifier.verify(signing_input.as_bytes(), &signature)?;

        if !valid {
            return Err(TokenSigningError::SigningFailed);
        }

        let payload = URL_SAFE_NO_PAD
            .decode(parts[1])
            .map_err(|_| TokenSigningError::SigningFailed)?;

        serde_json::from_slice(&payload).map_err(|_| TokenSigningError::SigningFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeVerifier;

    impl TokenVerifier for FakeVerifier {
        fn verify(&self, _payload: &[u8], _signature: &[u8]) -> Result<bool, TokenSigningError> {
            Ok(true)
        }
    }

    #[derive(Debug, serde::Deserialize, PartialEq)]
    struct Claims {
        sub: String,
    }

    #[test]
    fn should_verify_jwt_structure() {
        let verifier = JwtVerifier::new(FakeVerifier);

        let token = "e30.eyJzdWIiOiIxIn0.c2ln";

        let claims: Claims = verifier.verify(token).unwrap();

        assert_eq!(claims.sub, "1");
    }
}
