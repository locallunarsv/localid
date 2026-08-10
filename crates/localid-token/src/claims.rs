//! OpenID Connect ID token claims.

use serde::Serialize;

/// ID token claims.
#[derive(Debug, Clone, Serialize)]
pub struct IdTokenClaims {
    /// Issuer identifier.
    pub iss: String,

    /// Subject identifier.
    pub sub: String,

    /// Audience client identifier.
    pub aud: String,

    /// Issued at timestamp.
    pub iat: i64,

    /// Expiration timestamp.
    pub exp: i64,

    /// Optional nonce.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_id_token_claims() {
        let claims = IdTokenClaims {
            iss: "http://auth.home.arpa".into(),
            sub: "identity-1".into(),
            aud: "client-1".into(),
            iat: 1000,
            exp: 2000,
            nonce: None,
        };

        assert_eq!(claims.iss, "http://auth.home.arpa");
    }
}
