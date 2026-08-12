use axum::{ body::Body, http::Request };

use http_body_util::BodyExt;
use jsonwebtoken::{ decode, decode_header, Algorithm, DecodingKey, Validation };
use serde::Deserialize;
use serde_json::{ json, Value };
use tower::ServiceExt;

use localid_api::{ bootstrap::create_state, create_router };

#[derive(Debug, Deserialize)]
struct IdTokenClaims {
    iss: String,
    sub: String,
    aud: String,
    exp: i64,
    iat: i64,
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
async fn oidc_id_token_should_verify_signature_using_jwks() {
    let bootstrap = create_state();

    let oauth_client_id = bootstrap.oauth_client_public_id;
    let identity_id = bootstrap.identity_id;

    let app = create_router(bootstrap.state, bootstrap.auth_state, bootstrap.authorization_state);

    // Step 1: create authorization code
    let authorize_request = Request::builder()
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

    let authorize_response = app.clone().oneshot(authorize_request).await.unwrap();

    let location = authorize_response
        .headers()
        .get("location")
        .expect("location should exist")
        .to_str()
        .unwrap();

    let code = extract_authorization_code(location);

    // Step 2: exchange authorization code for tokens
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

    let token_response = app.clone().oneshot(token_request).await.unwrap();

    let token_body = token_response.into_body().collect().await.unwrap().to_bytes();

    let token_json: Value = serde_json::from_slice(&token_body).unwrap();

    let id_token = token_json["id_token"].as_str().expect("id_token should exist");

    // Step 3: decode JWT header
    let header = decode_header(id_token).expect("jwt header should decode");

    assert_eq!(header.alg, Algorithm::RS256);

    let kid = header.kid.expect("kid should exist");

    // Step 4: fetch JWKS
    let jwks_request = Request::builder()
        .method("GET")
        .uri("/.well-known/jwks.json")
        .body(Body::empty())
        .unwrap();

    let jwks_response = app.oneshot(jwks_request).await.unwrap();

    let jwks_body = jwks_response.into_body().collect().await.unwrap().to_bytes();

    let jwks_json: Value = serde_json::from_slice(&jwks_body).unwrap();

    let key = jwks_json["keys"]
        .as_array()
        .unwrap()
        .iter()
        .find(|key| key["kid"] == kid)
        .expect("matching jwk should exist");

    let n = key["n"].as_str().unwrap();

    let e = key["e"].as_str().unwrap();

    // Step 5: verify JWT signature and claims
    let mut validation = Validation::new(Algorithm::RS256);

    validation.set_issuer(&["http://localhost:8080"]);

    validation.validate_aud = false;

    let decoded = decode::<IdTokenClaims>(
        id_token,
        &DecodingKey::from_rsa_components(n, e).expect("rsa key should build"),
        &validation
    ).expect("id token signature should verify");

    assert_eq!(decoded.claims.iss, "http://localhost:8080");

    assert!(!decoded.claims.sub.is_empty());

    assert!(!decoded.claims.aud.is_empty());

    assert!(decoded.claims.exp > decoded.claims.iat);
}
