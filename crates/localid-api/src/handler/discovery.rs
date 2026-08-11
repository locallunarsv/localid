use axum::{extract::State, response::IntoResponse, Json};

use crate::{response::DiscoveryResponseBody, AppState};

/// Returns OpenID Connect discovery metadata.
pub async fn discovery<L, R, V, S, C, O, REX, TEX, ID, ITI>(
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX, ID, ITI>>,
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
{
    let issuer = state.config.issuer.as_str();

    Json(DiscoveryResponseBody {
        issuer: issuer.to_string(),
        authorization_endpoint: format!("{issuer}/oauth/authorize"),
        token_endpoint: format!("{issuer}/oauth/token"),
        userinfo_endpoint: format!("{issuer}/oauth/userinfo"),
        jwks_uri: format!("{issuer}/.well-known/jwks.json"),
        id_token_signing_alg_values_supported: vec!["RS256".to_string()],
    })
}
