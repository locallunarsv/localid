//! ID token issuance.

use localid_token::IdTokenClaims;

/// Issues OpenID Connect ID tokens.
pub trait IdTokenIssuer: Send + Sync {
    /// Issues an ID token.
    fn issue(&self, claims: IdTokenClaims) -> Result<String, IdTokenIssueError>;
}

/// ID token issuance error.
#[derive(Debug)]
pub enum IdTokenIssueError {
    /// Signing failed.
    SigningFailed,
}
