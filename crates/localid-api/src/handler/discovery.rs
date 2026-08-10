use axum::{response::IntoResponse, Json};

use crate::response::DiscoveryResponseBody;

/// Returns OpenID Connect discovery metadata.
pub async fn discovery() -> impl IntoResponse {
    Json(DiscoveryResponseBody {
        issuer: "http://localhost:8080".to_string(),
        authorization_endpoint: "http://localhost:8080/oauth/authorize".to_string(),
        token_endpoint: "http://localhost:8080/oauth/token".to_string(),
        userinfo_endpoint: "http://localhost:8080/oauth/userinfo".to_string(),
    })
}
