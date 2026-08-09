use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    Json,
};

use localid_application::{
    oauth::{authorization::AuthorizationPort, token_exchange::TokenExchangePort},
    AuthorizeCommand, IdentityLookupService, TokenExchangeCommand,
};

use localid_authentication::TokenIssuanceService;

use crate::{
    auth::AuthenticatedIdentity,
    request::{AuthorizeRequest, TokenRequest},
    response::{AuthorizeResponseBody, TokenResponseBody, UserInfoResponseBody},
    AppState,
};

/// Handles OAuth authorization request.
pub async fn authorize<L, R, V, S, C, O, REX, TEX, I>(
    Query(request): Query<AuthorizeRequest>,
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX, I>>,
) -> impl IntoResponse
where
    L: Send + Sync + 'static,
    R: Send + Sync + 'static,
    V: Send + Sync + 'static,
    S: Send + Sync + 'static,
    C: Send + Sync + 'static,
    O: AuthorizationPort + Send + Sync + 'static,
    REX: Send + Sync + 'static,
    TEX: Send + Sync + 'static,
    I: Send + Sync + 'static,
{
    let identity_id = match request.identity_id() {
        Ok(value) => value,
        Err(_) => {
            return Json(serde_json::json!({
                "error": "invalid_identity_id"
            }));
        }
    };

    let command = AuthorizeCommand::new(
        request.client_id(),
        identity_id,
        request.redirect_uri(),
        request.scope(),
    );

    let mut use_case = state.authorize_use_case.lock().await;

    match use_case.execute(command) {
        Ok(result) => Json(serde_json::json!(AuthorizeResponseBody::from(result))),

        Err(_) => Json(serde_json::json!({
            "error": "authorization_failed"
        })),
    }
}

/// Handles OAuth token exchange request.
pub async fn token<L, R, V, S, C, O, REX, TEX, I>(
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX, I>>,
    Json(request): Json<TokenRequest>,
) -> Response
where
    L: Send + Sync + 'static,
    R: Send + Sync + 'static,
    V: Send + Sync + 'static,
    S: Send + Sync + 'static,
    C: Send + Sync + 'static,
    O: Send + Sync + 'static,
    REX: TokenExchangePort + Send + Sync + 'static,
    TEX: TokenIssuanceService + Send + Sync + 'static,
    I: Send + Sync + 'static,
{
    let code_id = match request.code_id() {
        Ok(value) => value,

        Err(_) => {
            return Json(serde_json::json!({
                "error": "invalid_code_id"
            }))
            .into_response();
        }
    };

    let command = TokenExchangeCommand::new(code_id, request.client_id(), request.redirect_uri());

    let mut use_case = state.token_exchange_use_case.lock().await;

    match use_case.execute(command) {
        Ok(result) => Json(TokenResponseBody::from(result)).into_response(),

        Err(error) => {
            use localid_application::oauth::token_exchange::TokenExchangeError;

            match error {
                TokenExchangeError::AuthorizationCodeNotFound
                | TokenExchangeError::ClientNotFound
                | TokenExchangeError::ClientMismatch
                | TokenExchangeError::RedirectUriMismatch
                | TokenExchangeError::CodeExpired
                | TokenExchangeError::CodeConsumed => {
                    crate::error::ApiError::InvalidGrant.into_response()
                }

                TokenExchangeError::AuthorizationCodeRepositoryFailure
                | TokenExchangeError::OAuthClientRepositoryFailure
                | TokenExchangeError::TokenIssuanceFailure => {
                    crate::error::ApiError::InternalFailure.into_response()
                }
            }
        }
    }
}

/// Handles OAuth userinfo request.
pub async fn userinfo<L, R, V, S, C, O, REX, TEX, I>(
    AuthenticatedIdentity(context): AuthenticatedIdentity,
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX, I>>,
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
    TEX: Send + Sync + 'static,
    I: IdentityLookupService + Send + Sync + 'static,
{
    let mut use_case = state.identity_use_case.lock().await;

    match use_case.execute(context.identity_id()) {
        Ok(result) => Json(UserInfoResponseBody::from(result.identity().clone())).into_response(),

        Err(error) => crate::error::ApiError::from(error).into_response(),
    }
}
