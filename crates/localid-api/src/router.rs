use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use localid_application::{
    oauth::token_exchange::TokenExchangePort, AuthenticationPort, AuthorizationContextResolver,
    AuthorizationPort, ClientPort, IdentityRolePort, RefreshTokenPort, SessionPort,
};

use localid_authentication::{AuthenticationError, TokenIssuanceService, TokenVerificationService};

use tower_http::trace::TraceLayer;

use crate::middleware::request_id::request_id_layers;

use crate::{
    handler::{self, auth},
    middleware::{AuthMiddlewareState, AuthorizationMiddlewareState},
    AppState,
};

/// Creates the application HTTP router.
pub fn create_router<L, R, V, S, C, O, REX, TEX, IR>(
    state: AppState<L, R, V, S, C, O, REX, TEX>,
    auth_state: AuthMiddlewareState<V>,
    authorization_state: AuthorizationMiddlewareState<AuthorizationContextResolver<IR>>,
) -> Router
where
    L: AuthenticationPort<Error = AuthenticationError> + Send + Sync + 'static,
    R: RefreshTokenPort<Error = AuthenticationError> + Send + Sync + 'static,
    V: TokenVerificationService<Error = AuthenticationError> + Send + Sync + 'static,
    S: SessionPort<Error = AuthenticationError> + Send + Sync + 'static,
    C: ClientPort + Send + Sync + 'static,
    O: AuthorizationPort + Send + Sync + 'static,
    REX: TokenExchangePort + Send + Sync + 'static,
    TEX: TokenIssuanceService + Send + Sync + 'static,
    IR: IdentityRolePort + Send + Sync + 'static,
{
    let protected = Router::new()
        .route("/me", get(handler::me))
        .route("/session/current", get(handler::session::current))
        .route(
            "/authorization/context",
            get(handler::authorization::context),
        )
        .route(
            "/auth/logout",
            post(auth::logout::<L, R, V, S, C, O, REX, TEX>),
        )
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
        .route("/health", get(handler::health))
        .route(
            "/auth/login",
            post(auth::login::<L, R, V, S, C, O, REX, TEX>),
        )
        .route(
            "/auth/refresh",
            post(auth::refresh::<L, R, V, S, C, O, REX, TEX>),
        )
        .route(
            "/auth/verify",
            post(auth::verify::<L, R, V, S, C, O, REX, TEX>),
        )
        .route(
            "/oauth/authorize",
            get(handler::oauth::authorize::<L, R, V, S, C, O, REX, TEX>),
        )
        .route(
            "/oauth/token",
            post(handler::oauth::token::<L, R, V, S, C, O, REX, TEX>),
        )
        .merge(protected)
        .layer(TraceLayer::new_for_http())
        .layer(propagate_request_id_layer)
        .layer(request_id_layer)
        .with_state(state)
}
