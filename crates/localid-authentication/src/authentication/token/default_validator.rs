use chrono::Utc;
use localid_repository::{SessionRepository, TokenRepository};
use sha2::{Digest, Sha256};

use super::{AuthenticatedContext, TokenValidator};
use crate::AuthenticationError;

/// Default implementation of token validation.
///
/// Resolves a Token into an authenticated Session context.
pub struct DefaultTokenValidator<TR, SR> {
    token_repository: TR,
    session_repository: SR,
}

impl<TR, SR> DefaultTokenValidator<TR, SR> {
    /// Creates a new token validator.
    #[must_use]
    pub const fn new(token_repository: TR, session_repository: SR) -> Self {
        Self {
            token_repository,
            session_repository,
        }
    }

    fn hash_secret(secret: &str) -> String {
        let mut hasher = Sha256::new();

        hasher.update(secret.as_bytes());

        hex::encode(hasher.finalize())
    }
}

impl<TR, SR> TokenValidator for DefaultTokenValidator<TR, SR>
where
    TR: TokenRepository,
    SR: SessionRepository,
{
    type Error = AuthenticationError;

    fn validate(&self, secret: &str) -> Result<AuthenticatedContext, Self::Error> {
        let secret_hash = Self::hash_secret(secret);

        let token = self
            .token_repository
            .find_by_secret_hash(&secret_hash)
            .map_err(|_| AuthenticationError::TokenRepositoryFailure)?
            .ok_or(AuthenticationError::TokenNotFound)?;

        let now = Utc::now();

        if !token.is_valid_at(now) {
            return Err(AuthenticationError::TokenUnavailable);
        }

        let session = self
            .session_repository
            .find_by_id(token.session_id())
            .map_err(|_| AuthenticationError::SessionRepositoryFailure)?
            .ok_or(AuthenticationError::SessionNotFound)?;

        if !session.is_valid_at(now) {
            return Err(AuthenticationError::SessionUnavailable);
        }

        Ok(AuthenticatedContext::new(
            session.identity_id(),
            session.id(),
        ))
    }
}
