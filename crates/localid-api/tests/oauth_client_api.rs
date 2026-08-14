use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;

use localid_api::{bootstrap::create_state, create_router};
use localid_oauth_client::OAuthClientRepository;
use serde_json::{json, Value};
use tower::ServiceExt;

#[tokio::test]
async fn oauth_client_create_should_return_credentials() {
    let bootstrap = create_state();

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let request = Request::builder()
        .method("POST")
        .uri("/oauth/clients")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "name": "test-client",
                "redirect_uris": [
                    "http://localhost:3000/callback"
                ]
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["client_id"].is_string());
    assert!(json["client_secret"].is_string());
}

#[tokio::test]
async fn oauth_client_create_should_not_store_plain_secret() {
    let bootstrap = create_state();

    let repository = bootstrap.oauth_client_repository.clone();

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let request = Request::builder()
        .method("POST")
        .uri("/oauth/clients")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "name": "hash-test-client",
                "redirect_uris": [
                    "http://localhost:3000/callback"
                ]
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    let client_secret = json["client_secret"]
        .as_str()
        .expect("client secret should exist");

    let client_id = json["client_id"].as_str().expect("client id should exist");
    let stored_client = repository
        .find_by_client_id(client_id)
        .expect("repository lookup should succeed")
        .expect("client should exist");

    assert_ne!(stored_client.secret_hash(), client_secret);

    // nanti ambil dari repository
}
