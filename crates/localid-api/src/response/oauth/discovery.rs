/// OpenID Connect discovery response.
///
/// OpenID Connect discovery response.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DiscoveryResponseBody {
    /// Identity provider issuer URL.
    pub issuer: String,

    /// OAuth authorization endpoint.
    pub authorization_endpoint: String,

    /// OAuth token endpoint.
    pub token_endpoint: String,

    /// OpenID Connect userinfo endpoint.
    pub userinfo_endpoint: String,

    /// JSON Web Key Set endpoint.
    pub jwks_uri: String,

    /// Supported ID token signing algorithms.
    pub id_token_signing_alg_values_supported: Vec<String>,
}
