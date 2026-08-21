use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

mod common;

use common::{test_database, test_lock};
use localid_api::{
    bootstrap::{create_state, Environment},
    create_router,
};

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

#[tokio::test(flavor = "multi_thread")]
async fn oidc_token_should_issue_valid_id_token() {
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
                "client_secret": "demo-secret",
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

    let id_token = token_json["id_token"]
        .as_str()
        .expect("id_token should exist");

    let parts: Vec<&str> = id_token.split('.').collect();

    assert_eq!(parts.len(), 3);

    //
    // JWT Header
    //
    let header_bytes = URL_SAFE_NO_PAD
        .decode(parts[0])
        .expect("header should decode");

    let header: Value = serde_json::from_slice(&header_bytes).expect("header should be json");

    assert_eq!(header["alg"], "RS256");
    assert_eq!(header["kid"], "localid-key-1");

    //
    // JWT Signature
    //
    let signature = URL_SAFE_NO_PAD
        .decode(parts[2])
        .expect("signature should decode");

    assert!(!signature.is_empty());

    //
    // ID Token Claims
    //
    let payload_bytes = URL_SAFE_NO_PAD
        .decode(parts[1])
        .expect("payload should decode");

    let claims: Value = serde_json::from_slice(&payload_bytes).expect("claims should be json");

    assert_eq!(claims["iss"].as_str(), Some("http://localhost:8080"));

    assert!(claims["sub"].is_string());

    assert_eq!(claims["aud"].as_str(), Some(oauth_client_id.as_str()));

    assert_eq!(claims["nonce"].as_str(), Some("test-nonce"));

    assert!(claims["iat"].is_number());

    assert!(claims["exp"].is_number());
}
