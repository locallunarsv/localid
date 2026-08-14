use axum::{
    body::Body,
    http::{Request, StatusCode},
};

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
                "/oauth/authorize?client_id={}&identity_id={}&redirect_uri=http://localhost:3000/callback&response_type=code&scope=openid",
                oauth_client_id,
                identity_id
            )
        )
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);

    let location = response
        .headers()
        .get("location")
        .expect("location header should exist")
        .to_str()
        .unwrap();

    assert!(
        location.starts_with("http://localhost:3000/callback"),
        "should redirect to registered redirect uri"
    );

    assert!(
        location.contains("code="),
        "authorization code should exist"
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
            format!("/oauth/authorize?client_id=unknown-client&identity_id={}&redirect_uri=http://localhost:3000/callback&response_type=code&scope=openid", identity_id)
        )
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body();
    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();

    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(json["error"], "invalid_client");
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
                "/oauth/authorize?client_id={}&identity_id={}&redirect_uri=http://evil.com/callback&response_type=code&scope=openid",
                oauth_client_id,
                identity_id
            )
        )
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body();
    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();

    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(json["error"], "invalid_request");
}

#[tokio::test]
async fn oauth_authorization_should_preserve_state() {
    let bootstrap = create_state();

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let request = Request::builder()
        .method("GET")
        .uri(
            format!(
                "/oauth/authorize?client_id={}&identity_id={}&redirect_uri=http://localhost:3000/callback&response_type=code&scope=openid&state=test-state",
                bootstrap.oauth_client_public_id,
                bootstrap.identity_id
            )
        )
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);

    let location = response
        .headers()
        .get("location")
        .expect("location header should exist")
        .to_str()
        .unwrap();

    assert!(
        location.contains("state=test-state"),
        "state should be preserved"
    );
}

#[tokio::test]
async fn oauth_authorization_should_reject_invalid_response_type() {
    let bootstrap = create_state();

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let request = Request::builder()
        .method("GET")
        .uri(
            format!(
                "/oauth/authorize?client_id={}&identity_id={}&redirect_uri=http://localhost:3000/callback&response_type=token&scope=openid",
                bootstrap.oauth_client_public_id,
                bootstrap.identity_id
            )
        )
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn oauth_authorize_should_reject_disabled_client() {
    let bootstrap = create_state();

    let oauth_client_internal_id = bootstrap.oauth_client_id.to_string();
    let oauth_client_public_id = bootstrap.oauth_client_public_id;
    let identity_id = bootstrap.identity_id;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let disable_request = Request::builder()
        .method("POST")
        .uri(format!(
            "/oauth/clients/{}/disable",
            oauth_client_internal_id
        ))
        .body(Body::empty())
        .unwrap();

    let disable_response = app.clone().oneshot(disable_request).await.unwrap();

    assert_eq!(disable_response.status(), StatusCode::NO_CONTENT);

    let authorize_request = Request::builder()
        .method("GET")
        .uri(
            format!(
                "/oauth/authorize?client_id={}&identity_id={}&redirect_uri=http://localhost:3000/callback&response_type=code&scope=openid",
                oauth_client_public_id,
                identity_id
            )
        )
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(authorize_request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body();

    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();

    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(json["error"], "invalid_client");
}
