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
}
