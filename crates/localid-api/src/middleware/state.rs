use std::sync::Arc;

use tokio::sync::Mutex;

use localid_application::VerifyTokenUseCase;

/// Authentication middleware state.
pub struct AuthMiddlewareState<V> {
    /// Token verification use case.
    pub verify_token_use_case: Arc<Mutex<VerifyTokenUseCase<V>>>,
}

impl<V> Clone for AuthMiddlewareState<V> {
    fn clone(&self) -> Self {
        Self {
            verify_token_use_case: Arc::clone(&self.verify_token_use_case),
        }
    }
}

impl<V> AuthMiddlewareState<V> {
    /// Creates authentication middleware state.
    #[must_use]
    pub fn new(verify_token_use_case: Arc<Mutex<VerifyTokenUseCase<V>>>) -> Self {
        Self {
            verify_token_use_case,
        }
    }
}
