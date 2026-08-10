use axum::{response::IntoResponse, Json};

use crate::response::JwksResponseBody;

/// Returns JSON Web Key Set metadata.
pub async fn jwks() -> impl IntoResponse {
    Json(JwksResponseBody { keys: Vec::new() })
}
