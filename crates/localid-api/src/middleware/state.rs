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

/// Authorization middleware state.
/// Authorization middleware state.
pub struct AuthorizationMiddlewareState<R> {
    /// Authorization context resolver.
    pub resolver: Arc<Mutex<R>>,
}

impl<R> Clone for AuthorizationMiddlewareState<R> {
    fn clone(&self) -> Self {
        Self {
            resolver: Arc::clone(&self.resolver),
        }
    }
}

impl<R> AuthorizationMiddlewareState<R> {
    /// Creates authorization middleware state.
    #[must_use]
    pub fn new(resolver: R) -> Self {
        Self {
            resolver: Arc::new(Mutex::new(resolver)),
        }
    }
}
