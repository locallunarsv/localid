use std::sync::Arc;

use tokio::sync::Mutex;

use localid_application::{
    AuthorizeUseCase, GetCurrentSessionUseCase, LoginUseCase, LogoutSessionUseCase,
    RefreshTokenUseCase, TokenExchangeUseCase, VerifyTokenUseCase,
};

/// Shared application state.
pub struct AppState<L, R, V, S, C, O, REX, TEX, I> {
    /// Login authentication use case.
    pub login_use_case: Arc<Mutex<LoginUseCase<L, C>>>,

    /// Refresh token use case.
    pub refresh_use_case: Arc<Mutex<RefreshTokenUseCase<R>>>,

    /// Token verification use case.
    pub verify_token_use_case: Arc<Mutex<VerifyTokenUseCase<V>>>,

    /// Current session use case.
    pub current_session_use_case: Arc<Mutex<GetCurrentSessionUseCase<S>>>,

    /// Logout session use case.
    pub logout_use_case: Arc<Mutex<LogoutSessionUseCase<S>>>,

    /// OAuth authorization use case.
    pub authorize_use_case: Arc<Mutex<AuthorizeUseCase<O>>>,

    /// OAuth token exchange use case.
    pub token_exchange_use_case: Arc<Mutex<TokenExchangeUseCase<REX, TEX>>>,

    /// Identity lookup use case.
    pub identity_use_case: Arc<Mutex<I>>,
}

impl<L, R, V, S, C, O, REX, TEX, I> Clone for AppState<L, R, V, S, C, O, REX, TEX, I> {
    fn clone(&self) -> Self {
        Self {
            login_use_case: Arc::clone(&self.login_use_case),
            refresh_use_case: Arc::clone(&self.refresh_use_case),
            verify_token_use_case: Arc::clone(&self.verify_token_use_case),
            current_session_use_case: Arc::clone(&self.current_session_use_case),
            logout_use_case: Arc::clone(&self.logout_use_case),
            authorize_use_case: Arc::clone(&self.authorize_use_case),
            token_exchange_use_case: Arc::clone(&self.token_exchange_use_case),
            identity_use_case: Arc::clone(&self.identity_use_case),
        }
    }
}

impl<L, R, V, S, C, O, REX, TEX, I> AppState<L, R, V, S, C, O, REX, TEX, I> {
    /// Creates shared application state.
    #[must_use]
    pub fn new(
        login_use_case: LoginUseCase<L, C>,
        refresh_use_case: RefreshTokenUseCase<R>,
        verify_token_use_case: Arc<Mutex<VerifyTokenUseCase<V>>>,
        current_session_use_case: GetCurrentSessionUseCase<S>,
        logout_use_case: LogoutSessionUseCase<S>,
        authorize_use_case: AuthorizeUseCase<O>,
        token_exchange_use_case: TokenExchangeUseCase<REX, TEX>,
        identity_use_case: I,
    ) -> Self {
        Self {
            login_use_case: Arc::new(Mutex::new(login_use_case)),
            refresh_use_case: Arc::new(Mutex::new(refresh_use_case)),
            verify_token_use_case,
            current_session_use_case: Arc::new(Mutex::new(current_session_use_case)),
            logout_use_case: Arc::new(Mutex::new(logout_use_case)),
            authorize_use_case: Arc::new(Mutex::new(authorize_use_case)),
            token_exchange_use_case: Arc::new(Mutex::new(token_exchange_use_case)),
            identity_use_case: Arc::new(Mutex::new(identity_use_case)),
        }
    }
}
