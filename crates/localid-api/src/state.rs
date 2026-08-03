use std::sync::Arc;

use tokio::sync::Mutex;

use localid_application::LoginUseCase;

/// Shared application state.
pub struct AppState<A> {
    /// Login authentication use case.
    pub login_use_case: Arc<Mutex<LoginUseCase<A>>>,
}

impl<A> Clone for AppState<A> {
    fn clone(&self) -> Self {
        Self {
            login_use_case: Arc::clone(&self.login_use_case),
        }
    }
}

impl<A> AppState<A> {
    /// Creates application state.
    #[must_use]
    pub fn new(login_use_case: LoginUseCase<A>) -> Self {
        Self {
            login_use_case: Arc::new(Mutex::new(login_use_case)),
        }
    }
}
