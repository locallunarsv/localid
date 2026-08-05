use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use localid_application::{AuthenticationPort, RefreshTokenPort, SessionPort};
use tower_http::trace::TraceLayer;

use localid_authentication::{AuthenticationError, TokenVerificationService};

use crate::middleware::request_id::request_id_layers;
use crate::{
    handler::{self, auth},
    middleware::AuthMiddlewareState,
    AppState,
};

/// Creates the application HTTP router.
pub fn create_router<L, R, V, S>(
    state: AppState<L, R, V, S>,
    auth_state: AuthMiddlewareState<V>,
) -> Router
where
    L: AuthenticationPort<Error = AuthenticationError> + Send + Sync + 'static,
    R: RefreshTokenPort<Error = AuthenticationError> + Send + Sync + 'static,
    V: TokenVerificationService<Error = AuthenticationError> + Send + Sync + 'static,
    S: SessionPort<Error = AuthenticationError> + Send + Sync + 'static,
{
    let protected = Router::new()
        .route("/me", get(handler::me))
        .route("/session/current", get(handler::session::current))
        .layer(middleware::from_fn_with_state(
            auth_state,
            crate::middleware::auth::require_auth,
        ));

    let (request_id_layer, propagate_request_id_layer) = request_id_layers();

    Router::new()
        .route("/auth/login", post(auth::login::<L, R, V, S>))
        .route("/auth/refresh", post(auth::refresh::<L, R, V, S>))
        .route("/auth/verify", post(auth::verify::<L, R, V, S>))
        .merge(protected)
        .layer(TraceLayer::new_for_http())
        .layer(propagate_request_id_layer)
        .layer(request_id_layer)
        .with_state(state)
}
