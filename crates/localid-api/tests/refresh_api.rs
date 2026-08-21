use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};

use tower::ServiceExt;

use localid_api::{bootstrap::create_state, create_router};
use localid_config::Environment;

mod common;

use common::{test_database, test_lock};

#[tokio::test(flavor = "multi_thread")]
async fn refresh_returns_new_tokens() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Development).await;

    let client_id = bootstrap
        .client_id
        .expect("development seed should provide client ID");

    let credential_id = bootstrap
        .credential_id
        .expect("development seed should provide credential ID");

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let login_response = app
        .clone()
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

    assert_eq!(login_response.status(), StatusCode::OK);

    let body = to_bytes(login_response.into_body(), usize::MAX)
        .await
        .unwrap();

    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    let refresh_token = json["refresh_token"]
        .as_str()
        .expect("refresh token should exist");

    let refresh_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/refresh")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "refresh_token":"{}"
                    }}"#,
                    refresh_token
                )))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(refresh_response.status(), StatusCode::OK);

    let body = to_bytes(refresh_response.into_body(), usize::MAX)
        .await
        .unwrap();

    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json["access_token"].is_string());
    assert!(json["refresh_token"].is_string());
}
