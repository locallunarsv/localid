use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

use localid_api::{bootstrap::create_state, create_router};

#[tokio::test]
async fn oauth_authorize_should_issue_authorization_code() {
    let bootstrap = create_state();

    let oauth_client_id = bootstrap.oauth_client_public_id;
    let identity_id = bootstrap.identity_id;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let request = Request::builder()
        .method("GET")
        .uri(
            format!(
                "/oauth/authorize?client_id={}&identity_id={}&redirect_uri=http://localhost:3000/callback&scope=openid",
                oauth_client_id,
                identity_id
            )
        )
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(
        json["code_id"].as_str().is_some(),
        "authorization code id should exist"
    );
}

#[tokio::test]
async fn oauth_authorize_should_reject_unknown_client() {
    let bootstrap = create_state();

    let identity_id = bootstrap.identity_id;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let request = Request::builder()
        .method("GET")
        .uri(
            format!("/oauth/authorize?client_id=unknown-client&identity_id={}&redirect_uri=http://localhost:3000/callback&scope=openid", identity_id)
        )
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["error"].as_str(), Some("authorization_failed"));
}

#[tokio::test]
async fn oauth_authorize_should_reject_invalid_redirect_uri() {
    let bootstrap = create_state();

    let oauth_client_id = bootstrap.oauth_client_public_id;
    let identity_id = bootstrap.identity_id;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let request = Request::builder()
        .method("GET")
        .uri(
            format!(
                "/oauth/authorize?client_id={}&identity_id={}&redirect_uri=http://evil.com/callback&scope=openid",
                oauth_client_id,
                identity_id
            )
        )
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["error"].as_str(), Some("authorization_failed"));
}
