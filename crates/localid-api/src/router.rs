use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use localid_application::{AuthenticationPort, RefreshTokenPort};

use localid_authentication::{AuthenticationError, TokenVerificationService};

use crate::{
    handler::{self, auth},
    middleware::AuthMiddlewareState,
    AppState,
};

/// Creates the application HTTP router.
pub fn create_router<L, R, V>(
    state: AppState<L, R, V>,
    auth_state: AuthMiddlewareState<V>,
) -> Router
where
    L: AuthenticationPort<Error = AuthenticationError> + Send + Sync + 'static,
    R: RefreshTokenPort<Error = AuthenticationError> + Send + Sync + 'static,
    V: TokenVerificationService<Error = AuthenticationError> + Send + Sync + 'static,
{
    let protected =
        Router::new()
            .route("/me", get(handler::me))
            .layer(middleware::from_fn_with_state(
                auth_state,
                crate::middleware::auth::require_auth,
            ));

    Router::new()
        .route("/auth/login", post(auth::login::<L, R, V>))
        .route("/auth/refresh", post(auth::refresh::<L, R, V>))
        .route("/auth/verify", post(auth::verify::<L, R, V>))
        .merge(protected)
        .with_state(state)
}
