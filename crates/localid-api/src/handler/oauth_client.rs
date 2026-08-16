use axum::{extract::State, response::IntoResponse, Json};

use localid_application::CreateOAuthClientCommand;
use localid_application::GetOAuthClientQuery;
use localid_oauth_client::OAuthClientId;
use localid_oauth_client::OAuthClientRepository;
use std::str::FromStr;

use localid_application::ActivateOAuthClientCommand;
use localid_application::DeleteOAuthClientCommand;
use localid_application::DisableOAuthClientCommand;

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
    OCM: OAuthClientRepository + Send + Sync + 'static,
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

/// Lists OAuth clients.
pub async fn list<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>(
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>>,
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
    OCM: OAuthClientRepository + Send + Sync + 'static,
{
    let use_case = state.list_oauth_clients_use_case.lock().await;

    match use_case.execute() {
        Ok(result) => {
            let clients = result
                .clients()
                .iter()
                .cloned()
                .map(crate::response::oauth::OAuthClientResponse::from)
                .collect();

            Json(crate::response::oauth::ListOAuthClientsResponseBody::new(
                clients,
            ))
            .into_response()
        }

        Err(_) => crate::ApiError::InternalFailure.into_response(),
    }
}

/// Gets OAuth client.
pub async fn get<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>(
    axum::extract::Path(client_id): axum::extract::Path<String>,
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>>,
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
    OCM: OAuthClientRepository + Send + Sync + 'static,
{
    let client_id = match OAuthClientId::from_str(&client_id) {
        Ok(value) => value,

        Err(_) => {
            return crate::ApiError::InvalidRequest.into_response();
        }
    };

    let query = GetOAuthClientQuery::new(client_id);

    let use_case = state.get_oauth_client_use_case.lock().await;

    match use_case.execute(query) {
        Ok(result) => Json(crate::response::oauth::GetOAuthClientResponseBody::from(
            result.client(),
        ))
        .into_response(),

        Err(_) => crate::ApiError::NotFound.into_response(),
    }
}

/// Disables OAuth client.
pub async fn disable<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>(
    axum::extract::Path(client_id): axum::extract::Path<String>,
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>>,
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
    OCM: OAuthClientRepository + Send + Sync + 'static,
{
    let client_id = match localid_oauth_client::OAuthClientId::from_str(&client_id) {
        Ok(value) => value,

        Err(_) => {
            return crate::ApiError::InvalidRequest.into_response();
        }
    };

    let command = DisableOAuthClientCommand::new(client_id);

    let mut use_case = state.disable_oauth_client_use_case.lock().await;

    match use_case.execute(command) {
        Ok(()) => axum::http::StatusCode::NO_CONTENT.into_response(),

        Err(error) => match error {
            localid_application::DisableOAuthClientError::NotFound => {
                crate::ApiError::NotFound.into_response()
            }

            localid_application::DisableOAuthClientError::AlreadyDeleted => {
                crate::ApiError::InvalidRequest.into_response()
            }

            localid_application::DisableOAuthClientError::RepositoryFailure => {
                crate::ApiError::InternalFailure.into_response()
            }
        },
    }
}

/// Activates OAuth client.
pub async fn activate<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>(
    axum::extract::Path(client_id): axum::extract::Path<String>,
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>>,
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
    OCM: OAuthClientRepository + Send + Sync + 'static,
{
    let client_id = match localid_oauth_client::OAuthClientId::from_str(&client_id) {
        Ok(value) => value,

        Err(_) => {
            return crate::ApiError::InvalidRequest.into_response();
        }
    };

    let command = ActivateOAuthClientCommand::new(client_id);

    let mut use_case = state.activate_oauth_client_use_case.lock().await;

    match use_case.execute(command) {
        Ok(()) => axum::http::StatusCode::NO_CONTENT.into_response(),

        Err(error) => match error {
            localid_application::ActivateOAuthClientError::NotFound => {
                crate::ApiError::NotFound.into_response()
            }

            localid_application::ActivateOAuthClientError::AlreadyDeleted => {
                crate::ApiError::InvalidRequest.into_response()
            }

            localid_application::ActivateOAuthClientError::RepositoryFailure => {
                crate::ApiError::InternalFailure.into_response()
            }
        },
    }
}
/// Deletes OAuth client.
pub async fn delete<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>(
    axum::extract::Path(client_id): axum::extract::Path<String>,
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>>,
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
    OCM: OAuthClientRepository + Send + Sync + 'static,
{
    let client_id = match localid_oauth_client::OAuthClientId::from_str(&client_id) {
        Ok(value) => value,

        Err(_) => {
            return crate::ApiError::InvalidRequest.into_response();
        }
    };

    let command = DeleteOAuthClientCommand::new(client_id);

    let mut use_case = state.delete_oauth_client_use_case.lock().await;

    match use_case.execute(command) {
        Ok(()) => axum::http::StatusCode::NO_CONTENT.into_response(),

        Err(error) => match error {
            localid_application::DeleteOAuthClientError::NotFound => {
                crate::ApiError::NotFound.into_response()
            }

            localid_application::DeleteOAuthClientError::AlreadyDeleted => {
                crate::ApiError::InvalidRequest.into_response()
            }

            localid_application::DeleteOAuthClientError::RepositoryFailure => {
                crate::ApiError::InternalFailure.into_response()
            }
        },
    }
}
