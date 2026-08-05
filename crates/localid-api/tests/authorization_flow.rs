use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

use localid_api::{bootstrap::create_state, create_router};

#[tokio::test]
async fn authorization_context_should_resolve_roles() {
    let bootstrap = create_state();

    let credential_id = bootstrap.credential_id;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let payload = serde_json::json!({
        "credential_id": credential_id.to_string(),
        "password": "correct-password"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/auth/login")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    let token = json["access_token"]
        .as_str()
        .expect("access token should exist");

    let request = Request::builder()
        .method("GET")
        .uri("/authorization/context")
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();

    let status = response.status();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    println!("authorization status: {}", status);
    println!("authorization body: {}", String::from_utf8_lossy(&body));

    assert_eq!(status, StatusCode::OK);

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["roles"], 1);
}
