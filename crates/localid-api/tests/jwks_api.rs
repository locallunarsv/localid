use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

use localid_api::{bootstrap::create_state, create_router};

#[tokio::test]
async fn jwks_should_return_key_set() {
    let bootstrap = create_state();

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

    assert!(json["keys"].is_array());
    assert_eq!(json["keys"].as_array().unwrap().len(), 1);

    assert_eq!(json["keys"][0]["kty"].as_str(), Some("RSA"));

    assert_eq!(json["keys"][0]["kid"].as_str(), Some("localid-key-1"));

    assert_eq!(json["keys"][0]["alg"].as_str(), Some("RS256"));

    assert!(json["keys"][0]["n"].as_str().is_some());

    assert!(json["keys"][0]["e"].as_str().is_some());
}
