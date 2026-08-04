use std::sync::Arc;

use tokio::sync::Mutex;

use localid_application::{LoginUseCase, RefreshTokenUseCase, VerifyTokenUseCase};

/// Shared application state.
pub struct AppState<L, R, V> {
    /// Login authentication use case.
    pub login_use_case: Arc<Mutex<LoginUseCase<L>>>,

    /// Refresh token use case.
    pub refresh_use_case: Arc<Mutex<RefreshTokenUseCase<R>>>,

    /// Token verification use case.
    pub verify_token_use_case: Arc<Mutex<VerifyTokenUseCase<V>>>,
}

impl<L, R, V> Clone for AppState<L, R, V> {
    fn clone(&self) -> Self {
        Self {
            login_use_case: Arc::clone(&self.login_use_case),
            refresh_use_case: Arc::clone(&self.refresh_use_case),
            verify_token_use_case: Arc::clone(&self.verify_token_use_case),
        }
    }
}

impl<L, R, V> AppState<L, R, V> {
    /// Creates application state.
    #[must_use]
    pub fn new(
        login_use_case: LoginUseCase<L>,
        refresh_use_case: RefreshTokenUseCase<R>,
        verify_token_use_case: VerifyTokenUseCase<V>,
    ) -> Self {
        Self {
            login_use_case: Arc::new(Mutex::new(login_use_case)),
            refresh_use_case: Arc::new(Mutex::new(refresh_use_case)),
            verify_token_use_case: Arc::new(Mutex::new(verify_token_use_case)),
        }
    }
}
