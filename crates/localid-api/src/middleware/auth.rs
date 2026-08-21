use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use localid_application::VerifyTokenQuery;
use localid_authentication::TokenVerificationService;

use crate::{context::IdentityContext, error::ApiError, middleware::AuthMiddlewareState};

/// Authentication middleware.
pub async fn require_auth<V>(
    State(state): State<AuthMiddlewareState<V>>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiError>
where
    V: TokenVerificationService<Error = localid_authentication::AuthenticationError>
        + Send
        + Sync
        + 'static,
{
    let token = bearer_token(&request).or_else(|| session_cookie(&request));

    let token = token.ok_or(ApiError::AuthenticationFailed)?;

    let query = VerifyTokenQuery::new(token);

    let mut use_case = state.verify_token_use_case.lock().await;

    let identity = use_case
        .execute(query)
        .map_err(|_| ApiError::AuthenticationFailed)?;

    let context = IdentityContext::new(identity.identity_id(), identity.session_id());

    request.extensions_mut().insert(context);

    Ok(next.run(request).await)
}

fn bearer_token(request: &Request) -> Option<&str> {
    request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
}

fn session_cookie(request: &Request) -> Option<&str> {
    let cookies = request
        .headers()
        .get("Cookie")
        .and_then(|header| header.to_str().ok())?;

    cookies.split(';').find_map(|cookie| {
        let (name, value) = cookie.trim().split_once('=')?;

        if name == "localid_session" {
            Some(value)
        } else {
            None
        }
    })
}
