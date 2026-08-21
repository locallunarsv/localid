use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use localid_api::{bootstrap::create_state, create_router};
use localid_config::Environment;

mod common;

use common::{test_database, test_lock};

#[tokio::test(flavor = "multi_thread")]
async fn oauth_userinfo_should_return_identity() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let client_id = bootstrap
        .client_id
        .expect("development seed should provide client ID");

    let credential_id = bootstrap
        .credential_id
        .expect("development seed should provide credential ID");

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let login_request = Request::builder()
        .method("POST")
        .uri("/auth/login")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "client_id": client_id.to_string(),
                "credential_id": credential_id.to_string(),
                "password": "correct-password"
            })
            .to_string(),
        ))
        .unwrap();

    let login_response = app.clone().oneshot(login_request).await.unwrap();

    assert_eq!(login_response.status(), StatusCode::OK);

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

    let userinfo_request = Request::builder()
        .method("GET")
        .uri("/oauth/userinfo")
        .header("authorization", format!("Bearer {access_token}"))
        .body(Body::empty())
        .unwrap();

    let userinfo_response = app.oneshot(userinfo_request).await.unwrap();

    assert_eq!(userinfo_response.status(), StatusCode::OK);

    let body = userinfo_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["status"].as_str(), Some("Active"));

    assert!(json["sub"].as_str().is_some(), "subject should exist");
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_userinfo_should_reject_invalid_token() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let request = Request::builder()
        .method("GET")
        .uri("/oauth/userinfo")
        .header("authorization", "Bearer invalid-token")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
