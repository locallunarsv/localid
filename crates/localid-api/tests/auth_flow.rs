use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

use localid_api::{bootstrap::create_state, create_router};

struct AuthTokens {
    access_token: String,
    refresh_token: String,
}

async fn login() -> AuthTokens {
    let context = create_state();

    let credential_id = context.credential_id;
    let client_id = context.client_id;

    let app = create_router(
        context.state,
        context.auth_state,
        context.authorization_state,
    );

    let payload = serde_json::json!({
        "client_id": client_id.to_string(),
        "credential_id": credential_id.to_string(),
        "password": "correct-password"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/auth/login")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    AuthTokens {
        access_token: json["access_token"]
            .as_str()
            .expect("access token should exist")
            .to_owned(),

        refresh_token: json["refresh_token"]
            .as_str()
            .expect("refresh token should exist")
            .to_owned(),
    }
}

#[tokio::test]
async fn login_flow_should_return_tokens() {
    let tokens = login().await;

    assert!(!tokens.access_token.is_empty());
    assert!(!tokens.refresh_token.is_empty());
}

#[tokio::test]
async fn verify_access_token_should_work() {
    let context = create_state();

    let credential_id = context.credential_id;
    let client_id = context.client_id;

    let app = create_router(
        context.state,
        context.auth_state,
        context.authorization_state,
    );

    let login_payload = serde_json::json!({
        "client_id": client_id.to_string(),
        "credential_id": credential_id.to_string(),
        "password": "correct-password"
    });

    let login_request = Request::builder()
        .method("POST")
        .uri("/auth/login")
        .header("content-type", "application/json")
        .body(Body::from(login_payload.to_string()))
        .unwrap();

    let login_response = app.clone().oneshot(login_request).await.unwrap();

    let login_body = login_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let login_json: Value = serde_json::from_slice(&login_body).unwrap();

    let access_token = login_json["access_token"]
        .as_str()
        .expect("access token should exist");

    let verify_payload = serde_json::json!({
        "token": access_token
    });

    let verify_request = Request::builder()
        .method("POST")
        .uri("/auth/verify")
        .header("content-type", "application/json")
        .body(Body::from(verify_payload.to_string()))
        .unwrap();

    let verify_response = app.oneshot(verify_request).await.unwrap();

    assert_eq!(verify_response.status(), StatusCode::OK);

    let body = verify_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["identity_id"].as_str().is_some());
    assert!(json["session_id"].as_str().is_some());
}

#[tokio::test]
async fn refresh_token_should_issue_new_tokens() {
    let context = create_state();

    let credential_id = context.credential_id;
    let client_id = context.client_id;

    let app = create_router(
        context.state,
        context.auth_state,
        context.authorization_state,
    );

    let login_payload = serde_json::json!({
        "client_id": client_id.to_string(),
        "credential_id": credential_id.to_string(),
        "password": "correct-password"
    });

    let login_request = Request::builder()
        .method("POST")
        .uri("/auth/login")
        .header("content-type", "application/json")
        .body(Body::from(login_payload.to_string()))
        .unwrap();

    let login_response = app.clone().oneshot(login_request).await.unwrap();

    let login_body = login_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let login_json: Value = serde_json::from_slice(&login_body).unwrap();

    let refresh_token = login_json["refresh_token"]
        .as_str()
        .expect("refresh token should exist");

    let refresh_payload = serde_json::json!({
        "refresh_token": refresh_token
    });

    let refresh_request = Request::builder()
        .method("POST")
        .uri("/auth/refresh")
        .header("content-type", "application/json")
        .body(Body::from(refresh_payload.to_string()))
        .unwrap();

    let refresh_response = app.oneshot(refresh_request).await.unwrap();

    assert_eq!(refresh_response.status(), StatusCode::OK);

    let body = refresh_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["access_token"].as_str().is_some());
    assert!(json["refresh_token"].as_str().is_some());
    assert!(json["expires_at"].as_str().is_some());
}

#[tokio::test]
async fn protected_route_requires_valid_token() {
    let context = create_state();

    let app = create_router(
        context.state,
        context.auth_state,
        context.authorization_state,
    );

    let request = Request::builder()
        .method("GET")
        .uri("/me")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
