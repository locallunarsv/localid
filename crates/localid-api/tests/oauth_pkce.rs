use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use localid_api::{bootstrap::create_state, create_router};

#[tokio::test]
async fn oauth_token_should_reject_invalid_pkce_verifier() {
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
                "/oauth/authorize?client_id={}&identity_id={}&redirect_uri=http://localhost:3000/callback&scope=openid&response_type=code&code_challenge=invalid_challenge&code_challenge_method=S256",
                oauth_client_id,
                identity_id
            )
        )
        .body(Body::empty())
        .unwrap();

    let authorize_response = app.clone().oneshot(authorize_request).await.unwrap();

    assert_eq!(authorize_response.status(), StatusCode::OK);

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

    let token_request = Request::builder()
        .method("POST")
        .uri("/oauth/token")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "code_id": code_id,
                "client_id": oauth_client_id,
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
