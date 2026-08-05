use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

use localid_authorization::AuthorizationContext;

/// Extract authorization context from request extensions.
#[derive(Debug, Clone)]
pub struct AuthorizedContext(pub AuthorizationContext);

impl<S> FromRequestParts<S> for AuthorizedContext
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let context = parts
            .extensions
            .get::<AuthorizationContext>()
            .cloned()
            .ok_or(StatusCode::FORBIDDEN)?;

        Ok(Self(context))
    }
}
