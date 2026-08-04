use axum::{routing::post, Router};

use localid_application::{AuthenticationPort, RefreshTokenPort};

use localid_authentication::AuthenticationError;

use crate::{handler::auth, AppState};

/// Creates the application HTTP router.
pub fn create_router<L, R, V>(state: AppState<L, R, V>) -> Router
where
    L: AuthenticationPort<Error = AuthenticationError> + Send + Sync + 'static,
    R: RefreshTokenPort<Error = AuthenticationError> + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    Router::new()
        .route("/auth/login", post(auth::login::<L, R, V>))
        .route("/auth/refresh", post(auth::refresh::<L, R, V>))
        .with_state(state)
}
