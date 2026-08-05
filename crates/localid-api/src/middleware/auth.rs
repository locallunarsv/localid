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
    let header = request
        .headers()
        .get("Authorization")
        .ok_or(ApiError::AuthenticationFailed)?;

    let value = header
        .to_str()
        .map_err(|_| ApiError::AuthenticationFailed)?;

    let token = value
        .strip_prefix("Bearer ")
        .ok_or(ApiError::AuthenticationFailed)?;

    let query = VerifyTokenQuery::new(token);

    let mut use_case = state.verify_token_use_case.lock().await;

    // let identity = use_case
    //     .execute(query)
    //     .map_err(|_| ApiError::AuthenticationFailed)?;
    let identity = use_case.execute(query).map_err(|error| {
        println!("VERIFY TOKEN ERROR: {:?}", error);
        ApiError::AuthenticationFailed
    })?;

    let context = IdentityContext::new(identity.identity_id(), identity.session_id());

    request.extensions_mut().insert(context);

    Ok(next.run(request).await)
}
