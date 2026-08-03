use std::sync::Arc;

use tokio::sync::Mutex;

use localid_application::{LoginUseCase, RefreshTokenUseCase};

/// Shared application state.
pub struct AppState<L, R> {
    /// Login authentication use case.
    pub login_use_case: Arc<Mutex<LoginUseCase<L>>>,

    /// Refresh token use case.
    pub refresh_use_case: Arc<Mutex<RefreshTokenUseCase<R>>>,
}

impl<L, R> Clone for AppState<L, R> {
    fn clone(&self) -> Self {
        Self {
            login_use_case: Arc::clone(&self.login_use_case),
            refresh_use_case: Arc::clone(&self.refresh_use_case),
        }
    }
}

impl<L, R> AppState<L, R> {
    /// Creates application state.
    #[must_use]
    pub fn new(login_use_case: LoginUseCase<L>, refresh_use_case: RefreshTokenUseCase<R>) -> Self {
        Self {
            login_use_case: Arc::new(Mutex::new(login_use_case)),
            refresh_use_case: Arc::new(Mutex::new(refresh_use_case)),
        }
    }
}
