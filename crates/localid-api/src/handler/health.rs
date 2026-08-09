use axum::Json;
use serde_json::json;

/// Health check endpoint.
pub async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok"
    }))
}
