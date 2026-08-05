use std::sync::Arc;

use tokio::sync::Mutex;

use localid_application::{
    GetCurrentSessionUseCase, LoginUseCase, RefreshTokenUseCase, VerifyTokenUseCase,
};

/// Shared application state.
pub struct AppState<L, R, V, S> {
    /// Login authentication use case.
    pub login_use_case: Arc<Mutex<LoginUseCase<L>>>,
    /// Refresh token use case.
    pub refresh_use_case: Arc<Mutex<RefreshTokenUseCase<R>>>,
    /// Token verification use case.
    pub verify_token_use_case: Arc<Mutex<VerifyTokenUseCase<V>>>,
    /// Current session use case.
    pub current_session_use_case: Arc<Mutex<GetCurrentSessionUseCase<S>>>,
}

impl<L, R, V, S> Clone for AppState<L, R, V, S> {
    fn clone(&self) -> Self {
        Self {
            login_use_case: Arc::clone(&self.login_use_case),
            refresh_use_case: Arc::clone(&self.refresh_use_case),
            verify_token_use_case: Arc::clone(&self.verify_token_use_case),
            current_session_use_case: Arc::clone(&self.current_session_use_case),
        }
    }
}

impl<L, R, V, S> AppState<L, R, V, S> {
    /// Creates shared application state from application use cases.
    #[must_use]
    pub fn new(
        login_use_case: LoginUseCase<L>,
        refresh_use_case: RefreshTokenUseCase<R>,
        verify_token_use_case: Arc<Mutex<VerifyTokenUseCase<V>>>,
        current_session_use_case: GetCurrentSessionUseCase<S>,
    ) -> Self {
        Self {
            login_use_case: Arc::new(Mutex::new(login_use_case)),
            refresh_use_case: Arc::new(Mutex::new(refresh_use_case)),
            verify_token_use_case,
            current_session_use_case: Arc::new(Mutex::new(current_session_use_case)),
        }
    }
}
