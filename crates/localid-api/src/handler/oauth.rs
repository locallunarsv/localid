use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    Json,
};

use localid_application::{
    oauth::{
        authorization::AuthorizationPort,
        token_exchange::{IdTokenIssuer, TokenExchangePort},
    },
    AuthorizeCommand, IdentityLookupService, RefreshTokenPort, TokenExchangeCommand,
};

use localid_authentication::TokenIssuanceService;

use crate::{
    auth::AuthenticatedIdentity,
    request::{AuthorizeRequest, TokenRequest},
    response::{AuthorizeResponseBody, TokenResponseBody, UserInfoResponseBody},
    AppState,
};

/// Handles OAuth authorization request.
/// Handles OAuth authorization request.
pub async fn authorize<L, R, V, S, C, O, REX, TEX, ID, ITI>(
    Query(request): Query<AuthorizeRequest>,
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX, ID, ITI>>,
) -> impl IntoResponse
where
    L: Send + Sync + 'static,
    R: RefreshTokenPort<Error = localid_authentication::AuthenticationError>
        + Send
        + Sync
        + 'static,
    V: Send + Sync + 'static,
    S: Send + Sync + 'static,
    C: Send + Sync + 'static,
    O: AuthorizationPort + Send + Sync + 'static,
    REX: Send + Sync + 'static,
    TEX: Send + Sync + 'static,
    ID: Send + Sync + 'static,
    ITI: IdTokenIssuer + Send + Sync + 'static,
{
    if request.response_type() != "code" {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "unsupported_response_type"
            })),
        );
    }

    let scopes = request.scope();

    let supported_scopes = ["openid", "profile", "email"];

    if scopes
        .iter()
        .any(|scope| !supported_scopes.contains(&scope.as_str()))
    {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "invalid_scope"
            })),
        );
    }

    let identity_id = match request.identity_id() {
        Ok(value) => value,

        Err(_) => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "invalid_identity_id"
                })),
            );
        }
    };

    let command = AuthorizeCommand::new_with_nonce(
        request.client_id(),
        identity_id,
        request.redirect_uri(),
        scopes,
        request.nonce().map(ToOwned::to_owned),
        request.state().map(ToOwned::to_owned),
    );

    let mut use_case = state.authorize_use_case.lock().await;

    match use_case.execute(command) {
        Ok(result) => (
            axum::http::StatusCode::OK,
            Json(serde_json::json!(AuthorizeResponseBody::from(result))),
        ),

        Err(_) => (
            axum::http::StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "authorization_failed"
            })),
        ),
    }
}

/// Handles OAuth token request.
pub async fn token<L, R, V, S, C, O, REX, TEX, ID, ITI>(
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX, ID, ITI>>,
    Json(request): Json<TokenRequest>,
) -> Response
where
    L: Send + Sync + 'static,
    R: RefreshTokenPort<Error = localid_authentication::AuthenticationError>
        + Send
        + Sync
        + 'static,
    V: Send + Sync + 'static,
    S: Send + Sync + 'static,
    C: Send + Sync + 'static,
    O: Send + Sync + 'static,
    REX: TokenExchangePort + Send + Sync + 'static,
    TEX: TokenIssuanceService + Send + Sync + 'static,
    ID: Send + Sync + 'static,
    ITI: IdTokenIssuer + Send + Sync + 'static,
{
    match request.grant_type() {
        "refresh_token" => {
            let refresh_token = match request.refresh_token() {
                Some(value) => value,
                None => {
                    return crate::error::ApiError::InvalidRequest.into_response();
                }
            };

            let mut use_case = state.refresh_use_case.lock().await;

            match use_case.execute(refresh_token) {
                Ok(result) => Json(TokenResponseBody::from(result)).into_response(),

                Err(error) => match error {
                    localid_application::ApplicationError::InternalFailure => {
                        crate::error::ApiError::InternalFailure.into_response()
                    }

                    _ => crate::error::ApiError::InvalidGrant.into_response(),
                },
            }
        }

        _ => {
            let code_id = match request.code_id() {
                Ok(value) => value,

                Err(_) => {
                    return Json(serde_json::json!({
                        "error": "invalid_code_id"
                    }))
                    .into_response();
                }
            };

            let redirect_uri = match request.redirect_uri() {
                Some(value) => value,

                None => {
                    return crate::error::ApiError::InvalidRequest.into_response();
                }
            };

            let command = TokenExchangeCommand::new(code_id, request.client_id(), redirect_uri);

            let mut use_case = state.token_exchange_use_case.lock().await;

            match use_case.execute(command) {
                Ok(result) => Json(TokenResponseBody::from(result)).into_response(),

                Err(error) => crate::error::ApiError::from(localid_error::OAuthError::from(error))
                    .into_response(),
            }
        }
    }
}

/// Handles OAuth userinfo request.
pub async fn userinfo<L, R, V, S, C, O, REX, TEX, ID, ITI>(
    AuthenticatedIdentity(context): AuthenticatedIdentity,
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX, ID, ITI>>,
) -> Response
where
    L: Send + Sync + 'static,
    R: Send + Sync + 'static,
    V: Send + Sync + 'static,
    S: Send + Sync + 'static,
    C: Send + Sync + 'static,
    O: Send + Sync + 'static,
    REX: Send + Sync + 'static,
    TEX: Send + Sync + 'static,
    ID: IdentityLookupService + Send + Sync + 'static,
    ITI: IdTokenIssuer + Send + Sync + 'static,
{
    let mut use_case = state.identity_use_case.lock().await;

    match use_case.execute(context.identity_id()) {
        Ok(result) => Json(UserInfoResponseBody::from(result.identity().clone())).into_response(),

        Err(error) => crate::error::ApiError::from(error).into_response(),
    }
}
