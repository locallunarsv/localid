use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

use localid_api::{bootstrap::create_state, create_router};

#[tokio::test]
async fn discovery_should_return_openid_configuration() {
    let bootstrap = create_state();

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let request = Request::builder()
        .method("GET")
        .uri("/.well-known/openid-configuration")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["issuer"].as_str(), Some("http://localhost:8080"));

    assert!(json["authorization_endpoint"].as_str().is_some());

    assert!(json["token_endpoint"].as_str().is_some());

    assert!(json["userinfo_endpoint"].as_str().is_some());

    assert_eq!(json["id_token_signing_alg_values_supported"][0], "RS256");
    assert!(json["jwks_uri"].as_str().is_some());

    assert_eq!(json["id_token_signing_alg_values_supported"][0], "RS256");
    assert_eq!(json["response_types_supported"][0], "code");

    assert!(json["scopes_supported"]
        .as_array()
        .unwrap()
        .contains(&serde_json::json!("openid")));

    assert!(json["grant_types_supported"]
        .as_array()
        .unwrap()
        .contains(&serde_json::json!("authorization_code")));
}
