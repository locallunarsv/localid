use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use localid_application::{AuthorizationContextResolver, IdentityRolePort};

use crate::{context::IdentityContext, error::ApiError, middleware::AuthorizationMiddlewareState};

/// Authorization context middleware.
pub async fn resolve_authorization<R>(
    State(state): State<AuthorizationMiddlewareState<AuthorizationContextResolver<R>>>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiError>
where
    R: IdentityRolePort + Send + Sync + 'static,
{
    let identity = request
        .extensions()
        .get::<IdentityContext>()
        .copied()
        .ok_or(ApiError::AuthenticationFailed)?;

    let resolver = state.resolver.lock().await;

    let context = resolver
        .resolve(identity.identity_id(), identity.session_id())
        .map_err(|_| ApiError::AuthenticationFailed)?;

    request.extensions_mut().insert(context);

    Ok(next.run(request).await)
}
