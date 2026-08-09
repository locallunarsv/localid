use axum::{extract::State, response::IntoResponse, Json};

use crate::{auth::AuthenticatedIdentity, response::SessionResponseBody, ApiError, AppState};

use localid_application::SessionPort;
use localid_authentication::AuthenticationError;

pub async fn current<L, R, V, S, C, O, REX, TEX>(
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX>>,
    AuthenticatedIdentity(identity): AuthenticatedIdentity,
) -> impl IntoResponse
where
    L: Send + Sync + 'static,
    R: Send + Sync + 'static,
    V: Send + Sync + 'static,
    S: SessionPort<Error = AuthenticationError> + Send + Sync + 'static,
    C: Send + Sync + 'static,
{
    let session_id = identity.session_id();

    let mut use_case = state.current_session_use_case.lock().await;

    match use_case.execute(session_id) {
        Ok(response) => Json(SessionResponseBody::from(response)).into_response(),

        Err(error) => ApiError::from(error).into_response(),
    }
}
