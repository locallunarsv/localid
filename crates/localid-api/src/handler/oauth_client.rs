use axum::{extract::State, response::IntoResponse, Json};

use localid_application::CreateOAuthClientCommand;
use localid_oauth_client::OAuthClientRepository;

use crate::{
    request::oauth::CreateOAuthClientRequest, response::oauth::CreateOAuthClientResponseBody,
    AppState,
};

/// Creates OAuth client.
pub async fn create<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>(
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>>,
    Json(request): Json<CreateOAuthClientRequest>,
) -> impl IntoResponse
where
    L: Send + Sync + 'static,
    R: Send + Sync + 'static,
    V: Send + Sync + 'static,
    S: Send + Sync + 'static,
    C: Send + Sync + 'static,
    O: Send + Sync + 'static,
    REX: Send + Sync + 'static,
    TEX: Send + Sync + 'static,
    ID: Send + Sync + 'static,
    ITI: Send + Sync + 'static,
    CA: Send + Sync + 'static,
    OCM: OAuthClientRepository<Error = ()> + Send + Sync + 'static,
{
    let command = CreateOAuthClientCommand::new(request.name(), request.redirect_uris().to_vec());

    let mut use_case = state.create_oauth_client_use_case.lock().await;

    match use_case.execute(command) {
        Ok(result) => Json(CreateOAuthClientResponseBody::new(
            result.client_id(),
            result.client_secret(),
        ))
        .into_response(),

        Err(_) => crate::ApiError::InternalFailure.into_response(),
    }
}
