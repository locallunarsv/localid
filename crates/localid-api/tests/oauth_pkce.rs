use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use tower::ServiceExt;

mod common;

use common::{test_database, test_lock};
use localid_api::{
    bootstrap::{create_state, Environment},
    create_router,
};

fn demo_client_secret() -> &'static str {
    "demo-secret"
}

fn generate_code_challenge(verifier: &str) -> String {
    let digest = Sha256::digest(verifier.as_bytes());

    URL_SAFE_NO_PAD.encode(digest)
}

fn extract_authorization_code(location: &str) -> String {
    location
        .split("code=")
        .nth(1)
        .and_then(|value| value.split('&').next())
        .expect("authorization code should exist")
        .to_string()
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

async fn create_pkce_authorization_code(
    app: &axum::Router,
    client_id: &str,
    session_cookie: &str,
    challenge: &str,
) -> String {
    let authorize_request = Request::builder()
        .method("GET")
        .uri(format!(
            "/oauth/authorize?client_id={}&redirect_uri=http://localhost:3000/callback&scope=openid&response_type=code&code_challenge={}&code_challenge_method=S256",
            client_id,
            challenge
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

    extract_authorization_code(location)
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_token_should_reject_invalid_pkce_verifier() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let credential_id = bootstrap
        .demo_seed
        .as_ref()
        .expect("demo seed should exist")
        .credential_id;

    let client_id = bootstrap.client_id;
    let oauth_client_id = bootstrap.oauth_client_public_id;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let session_cookie = login_session_cookie(&app, client_id, credential_id).await;

    let code = create_pkce_authorization_code(
        &app,
        &oauth_client_id,
        &session_cookie,
        "invalid_challenge",
    )
    .await;

    let token_request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "code": code,
                "client_id": oauth_client_id,
                "client_secret": demo_client_secret(),
                "redirect_uri": "http://localhost:3000/callback",
                "code_verifier": "wrong-verifier"
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.oneshot(token_request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["code"].as_str(), Some("invalid_grant"));
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_token_should_accept_valid_pkce_verifier() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let credential_id = bootstrap
        .demo_seed
        .as_ref()
        .expect("demo seed should exist")
        .credential_id;

    let client_id = bootstrap.client_id;
    let oauth_client_id = bootstrap.oauth_client_public_id;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let session_cookie = login_session_cookie(&app, client_id, credential_id).await;

    let verifier = "test-code-verifier-123456789";

    let challenge = generate_code_challenge(verifier);

    let code =
        create_pkce_authorization_code(&app, &oauth_client_id, &session_cookie, &challenge).await;

    let token_request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "code": code,
                "client_id": oauth_client_id,
                "client_secret": demo_client_secret(),
                "redirect_uri": "http://localhost:3000/callback",
                "code_verifier": verifier
            })
            .to_string(),
        ))
        .unwrap();

    let token_response = app.oneshot(token_request).await.unwrap();

    assert_eq!(token_response.status(), StatusCode::OK);
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_token_should_reject_missing_pkce_verifier() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let credential_id = bootstrap
        .demo_seed
        .as_ref()
        .expect("demo seed should exist")
        .credential_id;

    let client_id = bootstrap.client_id;
    let oauth_client_id = bootstrap.oauth_client_public_id;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let session_cookie = login_session_cookie(&app, client_id, credential_id).await;

    let verifier = "test-code-verifier-123456789";

    let challenge = generate_code_challenge(verifier);

    let code =
        create_pkce_authorization_code(&app, &oauth_client_id, &session_cookie, &challenge).await;

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

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["code"].as_str(), Some("invalid_grant"));
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_authorize_should_reject_invalid_pkce_method() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let credential_id = bootstrap
        .demo_seed
        .as_ref()
        .expect("demo seed should exist")
        .credential_id;

    let client_id = bootstrap.client_id;
    let oauth_client_id = bootstrap.oauth_client_public_id;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let session_cookie = login_session_cookie(&app, client_id, credential_id).await;

    let request = Request::builder()
        .method("GET")
        .uri(format!(
            "/oauth/authorize?client_id={}&redirect_uri=http://localhost:3000/callback&scope=openid&response_type=code&code_challenge=test&code_challenge_method=plain",
            oauth_client_id
        ))
        .header("cookie", session_cookie)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
