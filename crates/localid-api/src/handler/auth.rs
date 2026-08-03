use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use localid_application::LoginCommand;

use crate::{request::login::LoginRequest, response::login::LoginResponseBody, ApiError, AppState};

use localid_application::AuthenticationPort;

pub async fn login<A>(
    State(state): State<AppState<A>>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse
where
    A: AuthenticationPort<Error = localid_authentication::AuthenticationError> + Send + Sync,
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
