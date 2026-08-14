use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect, Response},
    Json,
};
use localid_application::ClientAuthenticationCommand;
use localid_application::{
    oauth::{
        authorization::{AuthorizationError, AuthorizationPort},
        token_exchange::{IdTokenIssuer, TokenExchangePort},
    },
    AuthorizeCommand, IdentityLookupService, RefreshTokenPort, TokenExchangeCommand,
};

use localid_authentication::TokenIssuanceService;

use crate::{
    auth::AuthenticatedIdentity,
    request::{AuthorizeRequest, TokenRequest},
    response::{TokenResponseBody, UserInfoResponseBody},
    AppState,
};

use crate::response::build_authorization_redirect;

/// Handles OAuth authorization request.
pub async fn authorize<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>(
    Query(request): Query<AuthorizeRequest>,
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>>,
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
    O: AuthorizationPort + Send + Sync + 'static,
    REX: Send + Sync + 'static,
    TEX: Send + Sync + 'static,
    ID: Send + Sync + 'static,
    ITI: IdTokenIssuer + Send + Sync + 'static,
    CA: Send + Sync + 'static,
    OCM: Send + Sync + 'static,
{
    if request.response_type() != "code" {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "unsupported_response_type"
            })),
        )
            .into_response();
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
        )
            .into_response();
    }

    let identity_id = match request.identity_id() {
        Ok(value) => value,

        Err(_) => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "invalid_identity_id"
                })),
            )
                .into_response();
        }
    };

    let code_challenge_method = match request.code_challenge_method() {
        Some(value) => match localid_oauth_authorization::CodeChallengeMethod::from_str(value) {
            Some(method) => Some(method),

            None => {
                return (
                    axum::http::StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": "invalid_code_challenge_method"
                    })),
                )
                    .into_response();
            }
        },

        None => None,
    };

    let command = AuthorizeCommand::new_with_nonce_and_pkce(
        request.client_id(),
        identity_id,
        request.redirect_uri(),
        scopes,
        request.nonce().map(ToOwned::to_owned),
        request.state().map(ToOwned::to_owned),
        request.code_challenge().map(ToOwned::to_owned),
        code_challenge_method,
    );

    let mut use_case = state.authorize_use_case.lock().await;

    match use_case.execute(command) {
        Ok(result) => {
            let location = build_authorization_redirect(request.redirect_uri(), &result);

            Redirect::temporary(&location).into_response()
        }

        Err(error) => match error {
            AuthorizationError::InvalidClient => (
                axum::http::StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "invalid_client"
                })),
            )
                .into_response(),

            AuthorizationError::InvalidRedirectUri => (
                axum::http::StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "invalid_request",
                    "error_description": "invalid redirect uri"
                })),
            )
                .into_response(),

            AuthorizationError::InternalFailure => {
                crate::error::ApiError::InternalFailure.into_response()
            }
        },
    }
}

/// Handles OAuth token request.
pub async fn token<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>(
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>>,
    headers: axum::http::HeaderMap,
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
    CA: localid_application::ClientAuthenticationPort + Send + Sync + 'static,
    OCM: Send + Sync + 'static,
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
            let code = match request.code() {
                Some(value) => value,

                None => {
                    return crate::error::ApiError::InvalidRequest.into_response();
                }
            };

            let redirect_uri = match request.redirect_uri() {
                Some(value) => value,

                None => {
                    return crate::error::ApiError::InvalidRequest.into_response();
                }
            };

            let basic_credentials = TokenRequest::client_basic_credentials(
                headers
                    .get("authorization")
                    .and_then(|value| value.to_str().ok()),
            );

            let (client_id, client_secret) = match basic_credentials {
                Some(credentials) => credentials,

                None => match request.client_secret() {
                    Some(secret) => {
                        let client_id = match request.client_id() {
                            Some(value) => value.to_string(),

                            None => {
                                return crate::error::ApiError::InvalidRequest.into_response();
                            }
                        };

                        (client_id, secret.to_string())
                    }

                    None => {
                        return crate::error::ApiError::InvalidRequest.into_response();
                    }
                },
            };

            let client_auth_command =
                ClientAuthenticationCommand::new(client_id.clone(), client_secret);

            let client_auth_use_case = state.client_authentication_use_case.lock().await;

            if client_auth_use_case.execute(client_auth_command).is_err() {
                return crate::error::ApiError::InvalidGrant.into_response();
            }

            let command = TokenExchangeCommand::new(
                code.to_owned(),
                client_id,
                redirect_uri,
                request.code_verifier().map(ToOwned::to_owned),
            );

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
pub async fn userinfo<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>(
    AuthenticatedIdentity(context): AuthenticatedIdentity,
    State(state): State<AppState<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>>,
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
    CA: Send + Sync + 'static,
    OCM: Send + Sync + 'static,
{
    let mut use_case = state.identity_use_case.lock().await;

    match use_case.execute(context.identity_id()) {
        Ok(result) => Json(UserInfoResponseBody::from(result.identity().clone())).into_response(),

        Err(error) => crate::error::ApiError::from(error).into_response(),
    }
}
