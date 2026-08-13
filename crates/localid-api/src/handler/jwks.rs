use axum::{extract::State, response::IntoResponse, Json};

use crate::{response::JwksResponseBody, AppState};

/// Returns JSON Web Key Set metadata.
pub async fn jwks<L, R, V, S, C, O, REX, TEX, ID, ITI, CA>(
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX, ID, ITI, CA>>,
) -> impl IntoResponse {
    let jwk = state.signing_key.to_jwk();

    Json(JwksResponseBody { keys: vec![jwk] })
}
