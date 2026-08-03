use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use localid_application::{AuthenticationPort, LoginCommand, RefreshTokenPort};

use crate::{
    request::{LoginRequest, RefreshRequest},
    response::login::LoginResponseBody,
    ApiError, AppState,
};

pub async fn login<L, R>(
    State(state): State<AppState<L, R>>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse
where
    L: AuthenticationPort<Error = localid_authentication::AuthenticationError>
        + Send
        + Sync
        + 'static,
    R: Send + Sync + 'static,
{
    let credential_id = match request.credential_id() {
        Ok(value) => value,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, "invalid credential id").into_response();
        }
    };

    let password = match request.password() {
        Ok(value) => value,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, "invalid password").into_response();
        }
    };

    let command = LoginCommand::new(credential_id, password);

    let mut use_case = state.login_use_case.lock().await;

    match use_case.execute(command) {
        Ok(response) => Json(LoginResponseBody::from(response)).into_response(),

        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn refresh<L, R>(
    State(state): State<AppState<L, R>>,
    Json(request): Json<RefreshRequest>,
) -> impl IntoResponse
where
    L: Send + Sync + 'static,
    R: RefreshTokenPort<Error = localid_authentication::AuthenticationError>
        + Send
        + Sync
        + 'static,
{
    let mut use_case = state.refresh_use_case.lock().await;

    match use_case.execute(request.refresh_token()) {
        Ok(response) => Json(LoginResponseBody::from(response)).into_response(),

        Err(error) => ApiError::from(error).into_response(),
    }
}
