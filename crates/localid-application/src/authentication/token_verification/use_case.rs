use super::{query::VerifyTokenQuery, result::VerifyTokenResponse};

use localid_authentication::{TokenVerificationResult, TokenVerificationService};

use crate::ApplicationError;

/// Token verification application use case.
#[derive(Debug, Clone, Copy)]
pub struct VerifyTokenUseCase<S> {
    token_service: S,
}

impl<S> VerifyTokenUseCase<S>
where
    S: TokenVerificationService<Error = localid_authentication::AuthenticationError>,
{
    /// Creates a token verification use case.
    #[must_use]
    pub const fn new(token_service: S) -> Self {
        Self { token_service }
    }

    /// Executes token verification.
    pub fn execute(
        &mut self,
        query: VerifyTokenQuery,
    ) -> Result<VerifyTokenResponse, ApplicationError> {
        self.token_service
            .verify(query.token())
            .map(map_result)
            .map_err(|_| ApplicationError::AuthenticationFailed)
    }
}

fn map_result(result: TokenVerificationResult) -> VerifyTokenResponse {
    VerifyTokenResponse::new(result.identity_id(), result.session_id())
}
