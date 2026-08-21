use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::{engine::general_purpose::STANDARD, Engine};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use localid_api::{bootstrap::create_state, create_router};
use localid_config::Environment;

mod common;

use common::{test_database, test_lock};

fn decode_jwt_payload(token: &str) -> Value {
    let payload = token.split('.').nth(1).expect("jwt payload should exist");

    let decoded = URL_SAFE_NO_PAD
        .decode(payload)
        .expect("jwt payload should decode");

    serde_json::from_slice(&decoded).expect("payload should be json")
}

fn demo_client_secret() -> &'static str {
    "demo-secret"
}

async fn login_session_cookie(
    app: &axum::Router,
    client_id: impl std::fmt::Display,
    credential_id: impl std::fmt::Display,
) -> String {
    let payload = json!({
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

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    response
        .headers()
        .get("set-cookie")
        .expect("login should set session cookie")
        .to_str()
        .expect("set-cookie header should be valid")
        .split(';')
        .next()
        .expect("session cookie should exist")
        .to_owned()
}

fn extract_authorization_code(location: &str) -> String {
    location
        .split("code=")
        .nth(1)
        .and_then(|value| value.split('&').next())
        .expect("authorization code should exist")
        .to_string()
}

async fn create_authorization_code(
    app: &axum::Router,
    client_id: &str,
    session_cookie: &str,
) -> String {
    let request = Request::builder()
        .method("GET")
        .uri(format!(
            "/oauth/authorize?client_id={}&redirect_uri=http://localhost:3000/callback&scope=openid&response_type=code",
            client_id
        ))
        .header("cookie", session_cookie)
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);

    let location = response
        .headers()
        .get("location")
        .expect("location should exist")
        .to_str()
        .unwrap();

    extract_authorization_code(location)
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_token_should_exchange_authorization_code() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let credential_id = bootstrap
        .credential_id
        .expect("development seed should provide credential ID");

    let client_id = bootstrap
        .client_id
        .expect("development seed should provide client ID");

    let oauth_client_id = bootstrap
        .oauth_client_public_id
        .expect("development seed should provide OAuth client public ID");

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let session_cookie = login_session_cookie(&app, client_id, credential_id).await;

    let code = create_authorization_code(&app, &oauth_client_id, &session_cookie).await;

    let token_request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "code": code,
                "client_id": oauth_client_id,
                "client_secret": demo_client_secret(),
                "redirect_uri": "http://localhost:3000/callback"
            })
            .to_string(),
        ))
        .unwrap();

    let token_response = app.oneshot(token_request).await.unwrap();

    assert_eq!(token_response.status(), StatusCode::OK);

    let body = token_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["access_token"].as_str().is_some());
    assert!(json["refresh_token"].as_str().is_some());
    assert!(json["expires_at"].as_str().is_some());
    assert_eq!(json["token_type"].as_str(), Some("Bearer"));
    assert!(json["expires_in"].as_i64().is_some());
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_token_should_accept_basic_client_authentication() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let credential_id = bootstrap
        .credential_id
        .expect("development seed should provide credential ID");

    let client_id = bootstrap
        .client_id
        .expect("development seed should provide client ID");

    let oauth_client_id = bootstrap
        .oauth_client_public_id
        .expect("development seed should provide OAuth client public ID");

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let session_cookie = login_session_cookie(&app, client_id, credential_id).await;

    let code = create_authorization_code(&app, &oauth_client_id, &session_cookie).await;

    let credentials = format!("{}:{}", oauth_client_id, demo_client_secret());

    let encoded_credentials = STANDARD.encode(credentials);

    let token_request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .header("authorization", format!("Basic {}", encoded_credentials))
        .body(Body::from(
            json!({
                "code": code,
                "redirect_uri": "http://localhost:3000/callback"
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.oneshot(token_request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["access_token"].as_str().is_some());
    assert!(json["refresh_token"].as_str().is_some());
    assert_eq!(json["token_type"].as_str(), Some("Bearer"));
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_token_should_reject_invalid_code() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let oauth_client_id = bootstrap
        .oauth_client_public_id
        .expect("development seed should provide OAuth client public ID");

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
                "code": "invalid-authorization-code",
                "client_id": oauth_client_id,
                "client_secret": demo_client_secret(),
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

#[tokio::test(flavor = "multi_thread")]
async fn oauth_token_should_reject_reused_authorization_code() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let credential_id = bootstrap
        .credential_id
        .expect("development seed should provide credential ID");

    let client_id = bootstrap
        .client_id
        .expect("development seed should provide client ID");

    let oauth_client_id = bootstrap
        .oauth_client_public_id
        .expect("development seed should provide OAuth client public ID");

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let session_cookie = login_session_cookie(&app, client_id, credential_id).await;

    let code = create_authorization_code(&app, &oauth_client_id, &session_cookie).await;

    let first_request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "code": code,
                "client_id": oauth_client_id,
                "client_secret": demo_client_secret(),
                "redirect_uri": "http://localhost:3000/callback"
            })
            .to_string(),
        ))
        .unwrap();

    let first_response = app.clone().oneshot(first_request).await.unwrap();

    assert_eq!(first_response.status(), StatusCode::OK);

    let second_request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "code": code,
                "client_id": oauth_client_id,
                "client_secret": demo_client_secret(),
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

#[tokio::test(flavor = "multi_thread")]
async fn oauth_token_should_reject_client_mismatch() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let credential_id = bootstrap
        .credential_id
        .expect("development seed should provide credential ID");

    let client_id = bootstrap
        .client_id
        .expect("development seed should provide client ID");

    let oauth_client_id = bootstrap
        .oauth_client_public_id
        .expect("development seed should provide OAuth client public ID");

    let other_oauth_client_id = bootstrap
        .oauth_client_other_public_id
        .expect("development seed should provide secondary OAuth client public ID");

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let session_cookie = login_session_cookie(&app, client_id, credential_id).await;

    let code = create_authorization_code(&app, &oauth_client_id, &session_cookie).await;

    let request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "code": code,
                "client_id": other_oauth_client_id,
                "client_secret": demo_client_secret(),
                "redirect_uri": "http://localhost:3000/callback"
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["code"].as_str(), Some("invalid_grant"));
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_token_should_reject_invalid_client_secret() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let credential_id = bootstrap
        .credential_id
        .expect("development seed should provide credential ID");

    let client_id = bootstrap
        .client_id
        .expect("development seed should provide client ID");

    let oauth_client_id = bootstrap
        .oauth_client_public_id
        .expect("development seed should provide OAuth client public ID");

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let session_cookie = login_session_cookie(&app, client_id, credential_id).await;

    let code = create_authorization_code(&app, &oauth_client_id, &session_cookie).await;

    let request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "code": code,
                "client_id": oauth_client_id,
                "client_secret": "wrong-secret",
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

#[tokio::test(flavor = "multi_thread")]
async fn oauth_token_should_reject_redirect_uri_mismatch() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let credential_id = bootstrap
        .credential_id
        .expect("development seed should provide credential ID");

    let client_id = bootstrap
        .client_id
        .expect("development seed should provide client ID");

    let oauth_client_id = bootstrap
        .oauth_client_public_id
        .expect("development seed should provide OAuth client public ID");

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let session_cookie = login_session_cookie(&app, client_id, credential_id).await;

    let code = create_authorization_code(&app, &oauth_client_id, &session_cookie).await;

    let request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "code": code,
                "client_id": oauth_client_id,
                "client_secret": demo_client_secret(),
                "redirect_uri": "http://evil.com/callback"
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["code"].as_str(), Some("invalid_grant"));
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_token_should_refresh_access_token() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let credential_id = bootstrap
        .credential_id
        .expect("development seed should provide credential ID");

    let client_id = bootstrap
        .client_id
        .expect("development seed should provide client ID");

    let oauth_client_id = bootstrap
        .oauth_client_public_id
        .expect("development seed should provide OAuth client public ID");

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let session_cookie = login_session_cookie(&app, client_id, credential_id).await;

    let code = create_authorization_code(&app, &oauth_client_id, &session_cookie).await;

    let token_request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "code": code,
                "client_id": oauth_client_id,
                "client_secret": demo_client_secret(),
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
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_token_should_issue_id_token_for_openid_scope() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let credential_id = bootstrap
        .credential_id
        .expect("development seed should provide credential ID");

    let client_id = bootstrap
        .client_id
        .expect("development seed should provide client ID");

    let oauth_client_id = bootstrap
        .oauth_client_public_id
        .expect("development seed should provide OAuth client public ID");

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let session_cookie = login_session_cookie(&app, client_id, credential_id).await;

    let authorize_request = Request::builder()
        .method("GET")
        .uri(format!(
            "/oauth/authorize?client_id={}&redirect_uri=http://localhost:3000/callback&scope=openid&response_type=code&nonce=test-nonce",
            oauth_client_id
        ))
        .header("cookie", session_cookie)
        .body(Body::empty())
        .unwrap();

    let authorize_response = app.clone().oneshot(authorize_request).await.unwrap();

    assert_eq!(authorize_response.status(), StatusCode::TEMPORARY_REDIRECT);

    let location = authorize_response
        .headers()
        .get("location")
        .expect("location should exist")
        .to_str()
        .unwrap();

    let code = extract_authorization_code(location);

    let token_request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "code": code,
                "client_id": oauth_client_id,
                "client_secret": demo_client_secret(),
                "redirect_uri": "http://localhost:3000/callback"
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.oneshot(token_request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    let id_token = json["id_token"].as_str().expect("id_token should exist");

    let claims = decode_jwt_payload(id_token);

    assert_eq!(claims["iss"].as_str(), Some("http://localhost:8080"));
    assert_eq!(claims["aud"].as_str(), Some(oauth_client_id.as_str()));
    assert_eq!(claims["nonce"].as_str(), Some("test-nonce"));
    assert!(claims["sub"].as_str().is_some());
    assert!(claims["iat"].as_i64().is_some());
    assert!(claims["exp"].as_i64().is_some());
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_token_should_reject_missing_client_authentication() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let credential_id = bootstrap
        .credential_id
        .expect("development seed should provide credential ID");

    let client_id = bootstrap
        .client_id
        .expect("development seed should provide client ID");

    let oauth_client_id = bootstrap
        .oauth_client_public_id
        .expect("development seed should provide OAuth client public ID");

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let session_cookie = login_session_cookie(&app, client_id, credential_id).await;

    let code = create_authorization_code(&app, &oauth_client_id, &session_cookie).await;

    let request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "code": code,
                "redirect_uri": "http://localhost:3000/callback"
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_token_should_reject_invalid_basic_authentication() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let credential_id = bootstrap
        .credential_id
        .expect("development seed should provide credential ID");

    let client_id = bootstrap
        .client_id
        .expect("development seed should provide client ID");

    let oauth_client_id = bootstrap
        .oauth_client_public_id
        .expect("development seed should provide OAuth client public ID");

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let session_cookie = login_session_cookie(&app, client_id, credential_id).await;

    let code = create_authorization_code(&app, &oauth_client_id, &session_cookie).await;

    let credentials = format!("{}:{}", oauth_client_id, "wrong-secret");

    let encoded_credentials = STANDARD.encode(credentials);

    let request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .header("authorization", format!("Basic {}", encoded_credentials))
        .body(Body::from(
            json!({
                "code": code,
                "redirect_uri": "http://localhost:3000/callback"
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
