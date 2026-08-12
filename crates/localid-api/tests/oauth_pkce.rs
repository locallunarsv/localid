use axum::{ body::Body, http::{ Request, StatusCode } };

use base64::{ engine::general_purpose::URL_SAFE_NO_PAD, Engine };
use http_body_util::BodyExt;
use serde_json::{ json, Value };
use sha2::{ Digest, Sha256 };
use tower::ServiceExt;

use localid_api::{ bootstrap::create_state, create_router };

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

#[tokio::test]
async fn oauth_token_should_reject_invalid_pkce_verifier() {
    let bootstrap = create_state();

    let oauth_client_id = bootstrap.oauth_client_public_id;
    let identity_id = bootstrap.identity_id;

    let app = create_router(bootstrap.state, bootstrap.auth_state, bootstrap.authorization_state);

    let authorize_request = Request::builder()
        .method("GET")
        .uri(
            format!(
                "/oauth/authorize?client_id={}&identity_id={}&redirect_uri=http://localhost:3000/callback&scope=openid&response_type=code&code_challenge=invalid_challenge&code_challenge_method=S256",
                oauth_client_id,
                identity_id
            )
        )
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
        .body(
            Body::from(
                json!({
                "code": code,
                "client_id": oauth_client_id,
                "redirect_uri": "http://localhost:3000/callback",
                "code_verifier": "wrong-verifier"
            }).to_string()
            )
        )
        .unwrap();

    let response = app.oneshot(token_request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["code"].as_str(), Some("invalid_grant"));
}

#[tokio::test]
async fn oauth_token_should_accept_valid_pkce_verifier() {
    let bootstrap = create_state();

    let oauth_client_id = bootstrap.oauth_client_public_id;
    let identity_id = bootstrap.identity_id;

    let app = create_router(bootstrap.state, bootstrap.auth_state, bootstrap.authorization_state);

    let verifier = "test-code-verifier-123456789";

    let challenge = generate_code_challenge(verifier);

    let authorize_request = Request::builder()
        .method("GET")
        .uri(
            format!(
                "/oauth/authorize?client_id={}&identity_id={}&redirect_uri=http://localhost:3000/callback&scope=openid&response_type=code&code_challenge={}&code_challenge_method=S256",
                oauth_client_id,
                identity_id,
                challenge
            )
        )
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
        .body(
            Body::from(
                json!({
                "code": code,
                "client_id": oauth_client_id,
                "redirect_uri": "http://localhost:3000/callback",
                "code_verifier": verifier
            }).to_string()
            )
        )
        .unwrap();

    let token_response = app.oneshot(token_request).await.unwrap();

    assert_eq!(token_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn oauth_authorize_should_reject_invalid_pkce_method() {
    let bootstrap = create_state();

    let app = create_router(bootstrap.state, bootstrap.auth_state, bootstrap.authorization_state);

    let request = Request::builder()
        .method("GET")
        .uri(
            format!(
                "/oauth/authorize?client_id={}&identity_id={}&redirect_uri=http://localhost:3000/callback&scope=openid&response_type=code&code_challenge=test&code_challenge_method=plain",
                bootstrap.oauth_client_public_id,
                bootstrap.identity_id
            )
        )
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
