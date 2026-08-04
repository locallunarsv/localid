use axum::{extract::Extension, response::IntoResponse, Json};

use crate::context::IdentityContext;

/// Returns current authenticated identity.
pub async fn me(Extension(context): Extension<IdentityContext>) -> impl IntoResponse {
    Json(serde_json::json!({
        "identity_id": context.identity_id().to_string(),
        "session_id": context.session_id().to_string(),
    }))
}
