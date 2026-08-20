use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;

mod common;

use common::{test_database, test_lock};
use localid_api::{
    bootstrap::{create_state, Environment},
    create_router,
};

use localid_oauth_client::OAuthClientRepository;
use serde_json::{json, Value};
use tower::ServiceExt;

#[tokio::test(flavor = "multi_thread")]
async fn oauth_client_create_should_return_credentials() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

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

#[tokio::test(flavor = "multi_thread")]
async fn oauth_client_create_should_not_store_plain_secret() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

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

#[tokio::test(flavor = "multi_thread")]
async fn oauth_client_list_should_return_clients() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let request = Request::builder()
        .method("GET")
        .uri("/oauth/clients")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["clients"].is_array());
    assert!(!json["clients"].as_array().unwrap().is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_client_get_should_return_client() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let client_id = bootstrap.oauth_client_id.to_string();

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let request = Request::builder()
        .method("GET")
        .uri(format!("/oauth/clients/{client_id}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["client"]["state"], "active");

    assert!(json["client"]["client_id"].is_string());
    assert!(json["client"]["name"].is_string());
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_client_get_should_reject_unknown_client() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let request = Request::builder()
        .method("GET")
        .uri("/oauth/clients/01999999-9999-7999-8999-999999999999")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_client_disable_should_disable_client() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let client_id = bootstrap.oauth_client_id.to_string();

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let request = Request::builder()
        .method("POST")
        .uri(format!("/oauth/clients/{client_id}/disable"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_client_disable_should_reject_unknown_client() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let request = Request::builder()
        .method("POST")
        .uri("/oauth/clients/01999999-9999-7999-8999-999999999999/disable")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_client_delete_should_delete_client() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let client_id = bootstrap.oauth_client_id.to_string();

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let request = Request::builder()
        .method("POST")
        .uri(format!("/oauth/clients/{client_id}/delete"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_client_delete_should_reject_unknown_client() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let request = Request::builder()
        .method("POST")
        .uri("/oauth/clients/01999999-9999-7999-8999-999999999999/delete")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test(flavor = "multi_thread")]
async fn oauth_client_delete_should_reject_deleted_client() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let client_id = bootstrap.oauth_client_id.to_string();

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let request = Request::builder()
        .method("POST")
        .uri(format!("/oauth/clients/{client_id}/delete"))
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let request = Request::builder()
        .method("POST")
        .uri(format!("/oauth/clients/{client_id}/delete"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
