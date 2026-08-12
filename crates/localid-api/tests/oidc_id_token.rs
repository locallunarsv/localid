use axum::{ body::Body, http::{ Request, StatusCode } };

use base64::{ engine::general_purpose::URL_SAFE_NO_PAD, Engine };
use http_body_util::BodyExt;
use serde_json::{ json, Value };
use tower::ServiceExt;

use localid_api::{ bootstrap::create_state, create_router };

fn extract_authorization_code(location: &str) -> String {
    location
        .split("code=")
        .nth(1)
        .and_then(|value| value.split('&').next())
        .expect("authorization code should exist")
        .to_string()
}

#[tokio::test]
async fn oidc_token_should_return_valid_id_token() {
    let bootstrap = create_state();

    let oauth_client_id = bootstrap.oauth_client_public_id;
    let identity_id = bootstrap.identity_id;

    let app = create_router(bootstrap.state, bootstrap.auth_state, bootstrap.authorization_state);

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

    assert_eq!(authorize_response.status(), StatusCode::TEMPORARY_REDIRECT);

    let location = authorize_response
        .headers()
        .get("location")
        .expect("location should exist")
        .to_str()
        .unwrap();

    let code = extract_authorization_code(location);

    // Step 2: exchange authorization code
    let token_request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(
            Body::from(
                json!({
                "code": code,
                "client_id": oauth_client_id,
                "redirect_uri": "http://localhost:3000/callback"
            }).to_string()
            )
        )
        .unwrap();

    let token_response = app.oneshot(token_request).await.unwrap();

    assert_eq!(token_response.status(), StatusCode::OK);

    let token_body = token_response.into_body().collect().await.unwrap().to_bytes();

    let token_json: Value = serde_json::from_slice(&token_body).unwrap();

    let id_token = token_json["id_token"].as_str().expect("id_token should exist");

    // JWT structure
    let parts: Vec<&str> = id_token.split('.').collect();

    assert_eq!(parts.len(), 3);

    // JWT Header
    let header_bytes = URL_SAFE_NO_PAD.decode(parts[0]).expect("header should decode");

    let header: Value = serde_json::from_slice(&header_bytes).expect("header should be json");

    assert_eq!(header["alg"], "RS256");
    assert_eq!(header["kid"], "localid-key-1");

    // ID Token Claims
    let payload_bytes = URL_SAFE_NO_PAD.decode(parts[1]).expect("payload should decode");

    let claims: Value = serde_json::from_slice(&payload_bytes).expect("claims should be json");

    assert!(claims["iss"].is_string());
    assert!(claims["sub"].is_string());
    assert!(claims["aud"].is_string());

    assert!(claims["iat"].is_number());
    assert!(claims["exp"].is_number());
}
