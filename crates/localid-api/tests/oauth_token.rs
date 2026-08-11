use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use localid_api::{bootstrap::create_state, create_router};

#[tokio::test]
async fn oauth_token_should_exchange_authorization_code() {
    let bootstrap = create_state();

    let oauth_client_id = bootstrap.oauth_client_public_id;
    let identity_id = bootstrap.identity_id;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    // Step 1: create authorization code
    let authorize_request = Request::builder()
        .method("GET")
        .uri(
            format!(
                "/oauth/authorize?client_id={}&identity_id={}&redirect_uri=http://localhost:3000/callback&scope=openid&response_type=code",
                oauth_client_id,
                identity_id
            )
        )
        .body(Body::empty())
        .unwrap();

    let authorize_response = app.clone().oneshot(authorize_request).await.unwrap();

    assert_eq!(authorize_response.status(), StatusCode::OK);

    let authorize_body = authorize_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let authorize_json: Value = serde_json::from_slice(&authorize_body).unwrap();

    let code_id = authorize_json["code_id"]
        .as_str()
        .expect("authorization code id should exist");

    // Step 2: exchange authorization code
    let token_request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "code_id": code_id,
                "client_id": oauth_client_id,
                "redirect_uri": "http://localhost:3000/callback"
            })
            .to_string(),
        ))
        .unwrap();

    let token_response = app.oneshot(token_request).await.unwrap();

    assert_eq!(token_response.status(), StatusCode::OK);

    let token_body = token_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let token_json: Value = serde_json::from_slice(&token_body).unwrap();

    assert!(
        token_json["access_token"].as_str().is_some(),
        "access token should exist"
    );

    assert!(
        token_json["refresh_token"].as_str().is_some(),
        "refresh token should exist"
    );

    assert!(
        token_json["expires_at"].as_str().is_some(),
        "expires_at should exist"
    );

    assert_eq!(token_json["token_type"].as_str(), Some("Bearer"));

    assert!(
        token_json["expires_in"].as_i64().is_some(),
        "expires_in should exist"
    );
}

#[tokio::test]
async fn oauth_token_should_reject_invalid_code() {
    let bootstrap = create_state();

    let oauth_client_id = bootstrap.oauth_client_public_id;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "code_id": "00000000-0000-0000-0000-000000000000",
                "client_id": oauth_client_id,
                "redirect_uri": "http://localhost:3000/callback"
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["code"].as_str(), Some("invalid_grant"));
}

#[tokio::test]
async fn oauth_token_should_reject_reused_authorization_code() {
    let bootstrap = create_state();

    let oauth_client_id = bootstrap.oauth_client_public_id;
    let identity_id = bootstrap.identity_id;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    // create authorization code
    let authorize_request = Request::builder()
        .method("GET")
        .uri(
            format!(
                "/oauth/authorize?client_id={}&identity_id={}&redirect_uri=http://localhost:3000/callback&scope=openid&response_type=code",
                oauth_client_id,
                identity_id
            )
        )
        .body(Body::empty())
        .unwrap();

    let authorize_response = app.clone().oneshot(authorize_request).await.unwrap();

    let body = authorize_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    let code_id = json["code_id"]
        .as_str()
        .expect("authorization code should exist");

    // first exchange should succeed
    let first_request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "code_id": code_id,
                "client_id": oauth_client_id,
                "redirect_uri": "http://localhost:3000/callback"
            })
            .to_string(),
        ))
        .unwrap();

    let first_response = app.clone().oneshot(first_request).await.unwrap();

    assert_eq!(first_response.status(), StatusCode::OK);

    // second exchange with same code should fail
    let second_request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "code_id": code_id,
                "client_id": oauth_client_id,
                "redirect_uri": "http://localhost:3000/callback"
            })
            .to_string(),
        ))
        .unwrap();

    let second_response = app.oneshot(second_request).await.unwrap();

    let body = second_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["code"].as_str(), Some("invalid_grant"));
}

#[tokio::test]
async fn oauth_token_should_reject_client_mismatch() {
    let bootstrap = create_state();

    let oauth_client_id = bootstrap.oauth_client_public_id;
    let other_oauth_client_id = bootstrap.oauth_client_other_public_id;
    let identity_id = bootstrap.identity_id;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    // Create authorization code for registered client
    let authorize_request = Request::builder()
        .method("GET")
        .uri(
            format!(
                "/oauth/authorize?client_id={}&identity_id={}&redirect_uri=http://localhost:3000/callback&scope=openid&response_type=code",
                oauth_client_id,
                identity_id
            )
        )
        .body(Body::empty())
        .unwrap();

    let authorize_response = app.clone().oneshot(authorize_request).await.unwrap();

    let body = authorize_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    let code_id = json["code_id"]
        .as_str()
        .expect("authorization code should exist");

    // Exchange using different client id
    let token_request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "code_id": code_id,
                "client_id": other_oauth_client_id,
                "redirect_uri": "http://localhost:3000/callback"
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.oneshot(token_request).await.unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["code"].as_str(), Some("invalid_grant"));
}

#[tokio::test]
async fn oauth_token_should_reject_redirect_uri_mismatch() {
    let bootstrap = create_state();

    let oauth_client_id = bootstrap.oauth_client_public_id;
    let identity_id = bootstrap.identity_id;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let authorize_request = Request::builder()
        .method("GET")
        .uri(
            format!(
                "/oauth/authorize?client_id={}&identity_id={}&redirect_uri=http://localhost:3000/callback&scope=openid&response_type=code",
                oauth_client_id,
                identity_id
            )
        )
        .body(Body::empty())
        .unwrap();

    let authorize_response = app.clone().oneshot(authorize_request).await.unwrap();

    let body = authorize_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    let code_id = json["code_id"].as_str().unwrap();

    let token_request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "code_id": code_id,
                "client_id": oauth_client_id,
                "redirect_uri": "http://evil.com/callback"
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.oneshot(token_request).await.unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["code"].as_str(), Some("invalid_grant"));
}

#[tokio::test]
async fn oauth_token_should_refresh_access_token() {
    let bootstrap = create_state();

    let oauth_client_id = bootstrap.oauth_client_public_id;
    let identity_id = bootstrap.identity_id;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    // Step 1: create authorization code
    let authorize_request = Request::builder()
        .method("GET")
        .uri(
            format!(
                "/oauth/authorize?client_id={}&identity_id={}&redirect_uri=http://localhost:3000/callback&scope=openid&response_type=code",
                oauth_client_id,
                identity_id
            )
        )
        .body(Body::empty())
        .unwrap();

    let authorize_response = app.clone().oneshot(authorize_request).await.unwrap();

    let body = authorize_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    let code_id = json["code_id"]
        .as_str()
        .expect("authorization code should exist");

    // Step 2: exchange authorization code
    let token_request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "code_id": code_id,
                "client_id": oauth_client_id,
                "redirect_uri": "http://localhost:3000/callback"
            })
            .to_string(),
        ))
        .unwrap();

    let token_response = app.clone().oneshot(token_request).await.unwrap();

    assert_eq!(token_response.status(), StatusCode::OK);

    let body = token_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let token_json: Value = serde_json::from_slice(&body).unwrap();

    let refresh_token = token_json["refresh_token"]
        .as_str()
        .expect("refresh token should exist");

    // Step 3: refresh token
    let refresh_request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "grant_type": "refresh_token",
                "refresh_token": refresh_token,
                "client_id": oauth_client_id
            })
            .to_string(),
        ))
        .unwrap();

    let refresh_response = app.oneshot(refresh_request).await.unwrap();

    assert_eq!(refresh_response.status(), StatusCode::OK);

    let body = refresh_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let refresh_json: Value = serde_json::from_slice(&body).unwrap();

    assert!(
        refresh_json["access_token"].as_str().is_some(),
        "new access token should exist"
    );

    assert!(
        refresh_json["refresh_token"].as_str().is_some(),
        "new refresh token should exist"
    );

    assert_eq!(refresh_json["token_type"].as_str(), Some("Bearer"));
}
