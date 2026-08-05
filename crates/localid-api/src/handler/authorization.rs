use axum::{response::IntoResponse, Json};

use crate::auth::AuthorizedContext;

pub async fn context(AuthorizedContext(context): AuthorizedContext) -> impl IntoResponse {
    Json(serde_json::json!({
        "identity_id": context.identity_id().to_string(),
        "session_id": context.session_id().to_string(),
        "roles": context.roles().len(),
    }))
}
