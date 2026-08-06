use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use localid_application::{
    AuthenticationPort, AuthorizationContextResolver, IdentityRolePort, RefreshTokenPort,
    SessionPort,
};

use tower_http::trace::TraceLayer;

use localid_authentication::{AuthenticationError, TokenVerificationService};

use crate::middleware::request_id::request_id_layers;

use crate::{
    handler::{self, auth},
    middleware::{AuthMiddlewareState, AuthorizationMiddlewareState},
    AppState,
};

/// Creates the application HTTP router.
pub fn create_router<L, R, V, S, C, IR>(
    state: AppState<L, R, V, S, C>,
    auth_state: AuthMiddlewareState<V>,
    authorization_state: AuthorizationMiddlewareState<AuthorizationContextResolver<IR>>,
) -> Router
where
    L: AuthenticationPort<Error = AuthenticationError> + Send + Sync + 'static,
    R: RefreshTokenPort<Error = AuthenticationError> + Send + Sync + 'static,
    V: TokenVerificationService<Error = AuthenticationError> + Send + Sync + 'static,
    S: SessionPort<Error = AuthenticationError> + Send + Sync + 'static,
    C: Send + Sync + 'static,
    IR: IdentityRolePort + Send + Sync + 'static,
{
    let protected = Router::new()
        .route("/me", get(handler::me))
        .route("/session/current", get(handler::session::current))
        .route(
            "/authorization/context",
            get(handler::authorization::context),
        )
        .route("/auth/logout", post(auth::logout::<L, R, V, S, C>))
        .layer(middleware::from_fn_with_state(
            authorization_state,
            crate::middleware::authorization::resolve_authorization,
        ))
        .layer(middleware::from_fn_with_state(
            auth_state,
            crate::middleware::auth::require_auth,
        ));

    let (request_id_layer, propagate_request_id_layer) = request_id_layers();

    Router::new()
        .route("/auth/login", post(auth::login::<L, R, V, S, C>))
        .route("/auth/refresh", post(auth::refresh::<L, R, V, S, C>))
        .route("/auth/verify", post(auth::verify::<L, R, V, S, C>))
        .merge(protected)
        .layer(TraceLayer::new_for_http())
        .layer(propagate_request_id_layer)
        .layer(request_id_layer)
        .with_state(state)
}
