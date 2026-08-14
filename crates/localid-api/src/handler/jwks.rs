use axum::{extract::State, response::IntoResponse, Json};

use crate::{response::JwksResponseBody, AppState};

/// Returns JSON Web Key Set metadata.
pub async fn jwks<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>(
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>>,
) -> impl IntoResponse
where
    L: Send + Sync + 'static,
    R: Send + Sync + 'static,
    V: Send + Sync + 'static,
    S: Send + Sync + 'static,
    C: Send + Sync + 'static,
    O: Send + Sync + 'static,
    REX: Send + Sync + 'static,
    TEX: Send + Sync + 'static,
    ID: Send + Sync + 'static,
    ITI: Send + Sync + 'static,
    CA: Send + Sync + 'static,
    OCM: Send + Sync + 'static,
{
    let jwk = state.signing_key.to_jwk();

    Json(JwksResponseBody { keys: vec![jwk] })
}
