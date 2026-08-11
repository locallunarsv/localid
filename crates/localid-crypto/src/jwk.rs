//! JSON Web Key representation.

use serde::Serialize;

/// RSA JSON Web Key.
#[derive(Debug, Clone, Serialize)]
pub struct JsonWebKey {
    /// Key type.
    pub kty: String,

    /// Key identifier.
    pub kid: String,

    /// Key usage.
    #[serde(rename = "use")]
    pub use_: String,

    /// Algorithm.
    pub alg: String,

    /// RSA modulus.
    pub n: String,

    /// RSA exponent.
    pub e: String,
}
