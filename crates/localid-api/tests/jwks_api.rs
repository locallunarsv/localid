use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

use localid_api::{bootstrap::create_state, create_router};
use localid_config::Environment;

mod common;

use common::{test_database, test_lock};

#[tokio::test(flavor = "multi_thread")]
async fn jwks_should_return_key_set() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let request = Request::builder()
        .method("GET")
        .uri("/.well-known/jwks.json")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    let keys = json["keys"].as_array().expect("keys should be an array");

    assert_eq!(keys.len(), 1);

    let key = &keys[0];

    assert_eq!(key["kty"].as_str(), Some("RSA"));
    assert_eq!(key["kid"].as_str(), Some("localid-key-1"));
    assert_eq!(key["alg"].as_str(), Some("RS256"));
    assert_eq!(key["use"].as_str(), Some("sig"));

    assert!(key["n"].as_str().is_some_and(|value| !value.is_empty()));
    assert!(key["e"].as_str().is_some_and(|value| !value.is_empty()));
}
