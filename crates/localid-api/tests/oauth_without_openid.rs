use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

mod common;

use common::{test_database, test_lock};
use localid_api::{bootstrap::create_state, create_router};

fn extract_authorization_code(location: &str) -> String {
    location
        .split("code=")
        .nth(1)
        .and_then(|value| value.split('&').next())
        .expect("authorization code should exist")
        .to_string()
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_token_should_not_return_id_token_without_openid_scope() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database()).await;

    let oauth_client_id = bootstrap.oauth_client_public_id;
    let identity_id = bootstrap.identity_id;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    // Step 1: create authorization code without openid scope
    let authorize_request = Request::builder()
        .method("GET")
        .uri(format!(
            "/oauth/authorize?client_id={}&identity_id={}&redirect_uri=http://localhost:3000/callback&response_type=code&scope=profile",
            oauth_client_id,
            identity_id
        ))
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

    // Step 2: exchange token
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

    let status = token_response.status();

    if status != StatusCode::OK {
        let body = token_response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes();

        panic!(
            "token exchange failed: status={}, body={}",
            status,
            String::from_utf8_lossy(&body)
        );
    }

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
        token_json["id_token"].is_null(),
        "id_token should not exist without openid scope"
    );
}
