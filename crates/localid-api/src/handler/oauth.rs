use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};

use localid_application::{oauth::authorization::AuthorizationPort, AuthorizeCommand};

use crate::{request::AuthorizeRequest, response::AuthorizeResponseBody, AppState};

/// Handles OAuth authorization request.
pub async fn authorize<L, R, V, S, C, O>(
    Query(request): Query<AuthorizeRequest>,
    State(state): State<AppState<L, R, V, S, C, O>>,
) -> impl IntoResponse
where
    O: AuthorizationPort + Send + Sync + 'static,
{
    let identity_id = match request.identity_id() {
        Ok(value) => value,
        Err(_) => {
            return Json(serde_json::json!({
                "error": "invalid_identity_id"
            }));
        }
    };

    let command = AuthorizeCommand::new(
        request.client_id(),
        identity_id,
        request.redirect_uri(),
        request.scope(),
    );

    let mut use_case = state.authorize_use_case.lock().await;

    match use_case.execute(command) {
        Ok(result) => Json(serde_json::json!(AuthorizeResponseBody::from(result))),
        Err(_) => Json(serde_json::json!({
            "error": "authorization_failed"
        })),
    }
}
