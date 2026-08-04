use super::result::TokenVerificationResult;

/// Token verification service.
pub trait TokenVerificationService {
    /// Error returned during verification.
    type Error;

    /// Verifies an access token.
    fn verify(&mut self, token: &str) -> Result<TokenVerificationResult, Self::Error>;
}
