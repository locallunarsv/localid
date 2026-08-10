/// JSON Web Key Set response.
#[derive(Debug, Clone, serde::Serialize)]
pub struct JwksResponseBody {
    /// Public signing keys.
    pub keys: Vec<String>,
}
