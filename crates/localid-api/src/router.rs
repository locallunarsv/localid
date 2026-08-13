use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use localid_application::{
    oauth::token_exchange::{IdTokenIssuer, TokenExchangePort},
    AuthenticationPort, AuthorizationContextResolver, AuthorizationPort, ClientAuthenticationPort,
    ClientPort, IdentityLookupService, IdentityRolePort, RefreshTokenPort, SessionPort,
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
pub fn create_router<L, R, V, S, C, O, REX, TEX, ID, ITI, IR, CA>(
    state: AppState<L, R, V, S, C, O, REX, TEX, ID, ITI, CA>,
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
    ID: IdentityLookupService + Send + Sync + 'static,
    ITI: IdTokenIssuer + Send + Sync + 'static,
    CA: ClientAuthenticationPort + Send + Sync + 'static,
    IR: IdentityRolePort + Send + Sync + 'static,
{
    let protected = Router::new()
        .route("/me", get(handler::me))
        .route(
            "/oauth/userinfo",
            get(handler::oauth::userinfo::<L, R, V, S, C, O, REX, TEX, ID, ITI, CA>),
        )
        .route("/session/current", get(handler::session::current))
        .route(
            "/authorization/context",
            get(handler::authorization::context),
        )
        .route(
            "/auth/logout",
            post(auth::logout::<L, R, V, S, C, O, REX, TEX, ID, ITI, CA>),
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
        .route("/.well-known/openid-configuration", get(handler::discovery))
        .route("/.well-known/jwks.json", get(handler::jwks))
        .route(
            "/auth/login",
            post(auth::login::<L, R, V, S, C, O, REX, TEX, ID, ITI, CA>),
        )
        .route(
            "/auth/refresh",
            post(auth::refresh::<L, R, V, S, C, O, REX, TEX, ID, ITI, CA>),
        )
        .route(
            "/auth/verify",
            post(auth::verify::<L, R, V, S, C, O, REX, TEX, ID, ITI, CA>),
        )
        .route(
            "/oauth/authorize",
            get(handler::oauth::authorize::<L, R, V, S, C, O, REX, TEX, ID, ITI, CA>),
        )
        .route(
            "/oauth/token",
            post(handler::oauth::token::<L, R, V, S, C, O, REX, TEX, ID, ITI, CA>),
        )
        .merge(protected)
        .layer(TraceLayer::new_for_http())
        .layer(propagate_request_id_layer)
        .layer(request_id_layer)
        .with_state(state)
}
