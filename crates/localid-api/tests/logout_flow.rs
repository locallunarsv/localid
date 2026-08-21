use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use localid_api::{bootstrap::create_state, create_router};
use localid_config::Environment;

mod common;

use common::{test_database, test_lock};

#[tokio::test(flavor = "multi_thread")]
async fn logout_should_revoke_session_and_invalidate_token() {
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

    // login
    let login_request = Request::builder()
        .method("POST")
        .uri("/auth/login")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "client_id": client_id.to_string(),
                "credential_id": credential_id.to_string(),
                "password": "correct-password"
            })
            .to_string(),
        ))
        .unwrap();

    let login_response = app.clone().oneshot(login_request).await.unwrap();

    assert_eq!(login_response.status(), StatusCode::OK);

    let login_body = login_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let login_json: Value = serde_json::from_slice(&login_body).unwrap();

    let access_token = login_json["access_token"]
        .as_str()
        .expect("access token should exist");

    // verify token works before logout
    let me_request = Request::builder()
        .method("GET")
        .uri("/me")
        .header("authorization", format!("Bearer {access_token}"))
        .body(Body::empty())
        .unwrap();

    let me_response = app.clone().oneshot(me_request).await.unwrap();

    assert_eq!(me_response.status(), StatusCode::OK);

    // logout
    let logout_request = Request::builder()
        .method("POST")
        .uri("/auth/logout")
        .header("authorization", format!("Bearer {access_token}"))
        .body(Body::empty())
        .unwrap();

    let logout_response = app.clone().oneshot(logout_request).await.unwrap();

    assert_eq!(logout_response.status(), StatusCode::NO_CONTENT);

    // token should no longer work
    let expired_me_request = Request::builder()
        .method("GET")
        .uri("/me")
        .header("authorization", format!("Bearer {access_token}"))
        .body(Body::empty())
        .unwrap();

    let expired_me_response = app.oneshot(expired_me_request).await.unwrap();

    assert_eq!(expired_me_response.status(), StatusCode::UNAUTHORIZED);
}
