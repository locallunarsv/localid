use axum::{routing::post, Router};

use localid_application::AuthenticationPort;
use localid_authentication::AuthenticationError;

use crate::{handler::auth, AppState};

/// Creates the application HTTP router.
pub fn create_router<A>(state: AppState<A>) -> Router
where
    A: AuthenticationPort<Error = AuthenticationError> + Send + Sync + 'static,
{
    Router::new()
        .route("/auth/login", post(auth::login::<A>))
        .with_state(state)
}
