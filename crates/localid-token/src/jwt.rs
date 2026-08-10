//! JSON Web Token encoding utilities.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

use serde::Serialize;

use crate::{IdTokenClaims, TokenSigner, TokenSigningError};

/// JWT header.
#[derive(Debug, Serialize)]
pub struct JwtHeader {
    /// Signing algorithm.
    pub alg: &'static str,

    /// Key identifier.
    pub kid: String,

    /// Token type.
    pub typ: &'static str,
}

/// JWT encoder.
pub struct JwtEncoder<S> {
    signer: S,
    kid: String,
}

impl<S> JwtEncoder<S>
where
    S: TokenSigner,
{
    /// Creates a JWT encoder.
    #[must_use]
    pub fn new(signer: S, kid: impl Into<String>) -> Self {
        Self {
            signer,
            kid: kid.into(),
        }
    }

    /// Encodes ID token claims into JWT.
    pub fn encode(&self, claims: &IdTokenClaims) -> Result<String, TokenSigningError> {
        let header = JwtHeader {
            alg: "RS256",
            kid: self.kid.clone(),
            typ: "JWT",
        };

        let header_json =
            serde_json::to_vec(&header).map_err(|_| TokenSigningError::SigningFailed)?;

        let claims_json =
            serde_json::to_vec(claims).map_err(|_| TokenSigningError::SigningFailed)?;

        let encoded_header = URL_SAFE_NO_PAD.encode(header_json);

        let encoded_claims = URL_SAFE_NO_PAD.encode(claims_json);

        let signing_input = format!("{encoded_header}.{encoded_claims}");

        let signature = self.signer.sign(signing_input.as_bytes())?;

        let encoded_signature = URL_SAFE_NO_PAD.encode(signature);

        Ok(format!("{signing_input}.{encoded_signature}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeSigner;

    impl TokenSigner for FakeSigner {
        fn sign(&self, _data: &[u8]) -> Result<Vec<u8>, TokenSigningError> {
            Ok(b"signature".to_vec())
        }
    }

    #[test]
    fn should_create_jwt_structure() {
        let encoder = JwtEncoder::new(FakeSigner, "localid-key-1");

        let claims = IdTokenClaims {
            iss: "http://auth.home.arpa".into(),
            sub: "user-1".into(),
            aud: "client-1".into(),
            iat: 100,
            exp: 200,
            nonce: None,
        };

        let token = encoder.encode(&claims).unwrap();

        assert_eq!(token.split('.').count(), 3);
    }
}
