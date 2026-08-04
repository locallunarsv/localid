use chrono::Utc;
use sha2::{Digest, Sha256};

use localid_repository::{SessionRepository, TokenRepository};

use super::{result::TokenVerificationResult, service::TokenVerificationService};

use crate::AuthenticationError;

/// Default access token verification service.
pub struct DefaultTokenVerificationService<TR, SR> {
    token_repository: TR,
    session_repository: SR,
}

impl<TR, SR> DefaultTokenVerificationService<TR, SR> {
    /// Creates a new token verification service.
    #[must_use]
    pub const fn new(token_repository: TR, session_repository: SR) -> Self {
        Self {
            token_repository,
            session_repository,
        }
    }
}

fn hash_secret(secret: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(secret.as_bytes());

    hex::encode(hasher.finalize())
}

impl<TR, SR> TokenVerificationService for DefaultTokenVerificationService<TR, SR>
where
    TR: TokenRepository,
    SR: SessionRepository,
{
    type Error = AuthenticationError;

    fn verify(&mut self, token_secret: &str) -> Result<TokenVerificationResult, Self::Error> {
        let token_hash = hash_secret(token_secret);

        let token = self
            .token_repository
            .find_by_secret_hash(&token_hash)
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

        Ok(TokenVerificationResult::new(
            session.identity_id(),
            session.id(),
        ))
    }
}
