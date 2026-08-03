use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};

use tower::ServiceExt;

use localid_api::{bootstrap::create_state, create_router};

#[tokio::test]
async fn refresh_returns_new_tokens() {
    let bootstrap = create_state();

    let app = create_router(bootstrap.state);

    let login_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "credential_id":"{}",
                        "password":"correct-password"
                    }}"#,
                    bootstrap.credential_id
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

    let refresh_token = json["refresh_token"].as_str().unwrap();

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
