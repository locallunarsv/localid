use std::sync::Arc;

use tokio::sync::Mutex;

use localid_application::{
    GetClientUseCase, GetCurrentSessionUseCase, LoginUseCase, LogoutSessionUseCase,
    RefreshTokenUseCase, VerifyTokenUseCase,
};

/// Shared application state.
pub struct AppState<L, R, V, S, C> {
    /// Login authentication use case.
    pub login_use_case: Arc<Mutex<LoginUseCase<L>>>,

    /// Refresh token use case.
    pub refresh_use_case: Arc<Mutex<RefreshTokenUseCase<R>>>,

    /// Token verification use case.
    pub verify_token_use_case: Arc<Mutex<VerifyTokenUseCase<V>>>,

    /// Current session use case.
    pub current_session_use_case: Arc<Mutex<GetCurrentSessionUseCase<S>>>,

    /// Logout session use case.
    pub logout_use_case: Arc<Mutex<LogoutSessionUseCase<S>>>,

    /// Client lookup use case.
    pub client_use_case: Arc<Mutex<GetClientUseCase<C>>>,
}

impl<L, R, V, S, C> Clone for AppState<L, R, V, S, C> {
    fn clone(&self) -> Self {
        Self {
            login_use_case: Arc::clone(&self.login_use_case),
            refresh_use_case: Arc::clone(&self.refresh_use_case),
            verify_token_use_case: Arc::clone(&self.verify_token_use_case),
            current_session_use_case: Arc::clone(&self.current_session_use_case),
            logout_use_case: Arc::clone(&self.logout_use_case),
            client_use_case: Arc::clone(&self.client_use_case),
        }
    }
}

impl<L, R, V, S, C> AppState<L, R, V, S, C> {
    /// Creates shared application state from application use cases.
    #[must_use]
    pub fn new(
        login_use_case: LoginUseCase<L>,
        refresh_use_case: RefreshTokenUseCase<R>,
        verify_token_use_case: Arc<Mutex<VerifyTokenUseCase<V>>>,
        current_session_use_case: GetCurrentSessionUseCase<S>,
        logout_use_case: LogoutSessionUseCase<S>,
        client_use_case: GetClientUseCase<C>,
    ) -> Self {
        Self {
            login_use_case: Arc::new(Mutex::new(login_use_case)),
            refresh_use_case: Arc::new(Mutex::new(refresh_use_case)),
            verify_token_use_case,
            current_session_use_case: Arc::new(Mutex::new(current_session_use_case)),
            logout_use_case: Arc::new(Mutex::new(logout_use_case)),
            client_use_case: Arc::new(Mutex::new(client_use_case)),
        }
    }
}
