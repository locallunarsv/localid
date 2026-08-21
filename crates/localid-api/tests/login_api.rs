use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use tower::ServiceExt;

use localid_api::{
    bootstrap::{create_state, Environment},
    create_router,
};

mod common;

use common::{test_database, test_lock};

#[tokio::test(flavor = "multi_thread")]
async fn login_rejects_malformed_credential_id() {
    let _guard = test_lock().lock().await;
    let bootstrap = create_state(test_database(), Environment::Development).await;

    let client_id = bootstrap.client_id;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "client_id": "{}",
                        "credential_id": "invalid",
                        "password": "wrong-password"
                    }}"#,
                    client_id
                )))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test(flavor = "multi_thread")]
async fn login_returns_success_response() {
    let _guard = test_lock().lock().await;
    let bootstrap = create_state(test_database(), Environment::Development).await;

    let credential_id = bootstrap
        .demo_seed
        .as_ref()
        .expect("demo seed should exist")
        .credential_id;

    let client_id = bootstrap.client_id;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "client_id":"{}",
                        "credential_id":"{}",
                        "password":"correct-password"
                    }}"#,
                    client_id, credential_id
                )))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let set_cookie = response
        .headers()
        .get("set-cookie")
        .expect("login should set session cookie")
        .to_str()
        .expect("set-cookie header should be valid")
        .to_owned();

    assert!(set_cookie.contains("localid_session="));
    assert!(set_cookie.contains("HttpOnly"));
    assert!(set_cookie.contains("SameSite=Lax"));
    assert!(set_cookie.contains("Path=/"));

    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("response body should be readable");

    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");

    let access_token = json["access_token"]
        .as_str()
        .expect("access token should be a string");

    assert!(json["refresh_token"].is_string());
    assert!(json["expires_at"].is_string());

    assert!(
        set_cookie.contains(&format!("localid_session={access_token}")),
        "session cookie should contain the issued access token"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn login_rejects_invalid_password() {
    let _guard = test_lock().lock().await;
    let bootstrap = create_state(test_database(), Environment::Development).await;

    let credential_id = bootstrap
        .demo_seed
        .as_ref()
        .expect("demo seed should exist")
        .credential_id;

    let client_id = bootstrap.client_id;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "client_id":"{}",
                        "credential_id":"{}",
                        "password":"wrong-password"
                    }}"#,
                    client_id, credential_id
                )))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test(flavor = "multi_thread")]
async fn login_rejects_unknown_credential() {
    let _guard = test_lock().lock().await;
    let bootstrap = create_state(test_database(), Environment::Development).await;

    let client_id = bootstrap.client_id;
    let credential_id = localid_credential::CredentialId::new();

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "client_id":"{}",
                        "credential_id":"{}",
                        "password":"correct-password"
                    }}"#,
                    client_id, credential_id
                )))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
