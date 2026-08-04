use localid_authentication::{TokenVerificationResult, TokenVerificationService};

/// Adapter for token verification service.
#[derive(Debug, Clone, Copy)]
pub struct TokenVerificationAdapter<S> {
    service: S,
}

impl<S> TokenVerificationAdapter<S> {
    /// Creates a token verification adapter.
    #[must_use]
    pub const fn new(service: S) -> Self {
        Self { service }
    }
}

impl<S> TokenVerificationService for TokenVerificationAdapter<S>
where
    S: TokenVerificationService,
{
    type Error = S::Error;

    fn verify(&mut self, token: &str) -> Result<TokenVerificationResult, Self::Error> {
        self.service.verify(token)
    }
}
