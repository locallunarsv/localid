use axum::{extract::State, response::IntoResponse, Json};

use crate::{response::DiscoveryResponseBody, AppState};

/// Returns OpenID Connect discovery metadata.
pub async fn discovery<L, R, V, S, C, O, REX, TEX, I>(
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX, I>>,
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
    I: Send + Sync + 'static,
{
    let issuer = state.config.issuer();

    Json(DiscoveryResponseBody {
        issuer: issuer.to_string(),
        authorization_endpoint: format!("{issuer}/oauth/authorize"),
        token_endpoint: format!("{issuer}/oauth/token"),
        userinfo_endpoint: format!("{issuer}/oauth/userinfo"),
    })
}
