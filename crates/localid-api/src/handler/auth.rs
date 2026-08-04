use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use localid_application::{AuthenticationPort, LoginCommand, RefreshTokenPort, VerifyTokenQuery};

use localid_authentication::TokenVerificationService;

use crate::{
    request::{LoginRequest, RefreshRequest, VerifyTokenRequest},
    response::{LoginResponseBody, VerifyTokenResponseBody},
    ApiError, AppState,
};

pub async fn login<L, R, V>(
    State(state): State<AppState<L, R, V>>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse
where
    L: AuthenticationPort<Error = localid_authentication::AuthenticationError>
        + Send
        + Sync
        + 'static,
    R: Send + Sync + 'static,
    V: Send + Sync + 'static,
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

pub async fn refresh<L, R, V>(
    State(state): State<AppState<L, R, V>>,
    Json(request): Json<RefreshRequest>,
) -> impl IntoResponse
where
    L: Send + Sync + 'static,
    R: RefreshTokenPort<Error = localid_authentication::AuthenticationError>
        + Send
        + Sync
        + 'static,
    V: Send + Sync + 'static,
{
    let mut use_case = state.refresh_use_case.lock().await;

    match use_case.execute(request.refresh_token()) {
        Ok(response) => Json(LoginResponseBody::from(response)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn verify<L, R, V>(
    State(state): State<AppState<L, R, V>>,
    Json(request): Json<VerifyTokenRequest>,
) -> impl IntoResponse
where
    L: Send + Sync + 'static,
    R: Send + Sync + 'static,
    V: TokenVerificationService<Error = localid_authentication::AuthenticationError>
        + Send
        + Sync
        + 'static,
{
    let query = VerifyTokenQuery::new(request.token());

    let mut use_case = state.verify_token_use_case.lock().await;

    match use_case.execute(query) {
        Ok(response) => Json(VerifyTokenResponseBody::from(response)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}
