use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

use crate::context::IdentityContext;

/// Extract authenticated identity from request extensions.
#[derive(Debug, Clone, Copy)]
pub struct AuthenticatedIdentity(pub IdentityContext);

impl<S> FromRequestParts<S> for AuthenticatedIdentity
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let context = parts
            .extensions
            .get::<IdentityContext>()
            .cloned()
            .ok_or(StatusCode::UNAUTHORIZED)?;

        Ok(Self(context))
    }
}
