use sha2::{ Digest, Sha256 };

use localid_refresh_token_random::RefreshTokenIssuer;
use localid_repository::{ RefreshTokenRepository, SessionRepository, TokenRepository };
use localid_token_random::TokenIssuer;

use super::{ result::RefreshResult, service::RefreshTokenService };
use crate::AuthenticationError;

/// Default refresh token service.
pub struct DefaultRefreshTokenService<RTR, TR, SR, RTI, TI> {
    refresh_token_repository: RTR,
    token_repository: TR,
    session_repository: SR,
    refresh_token_issuer: RTI,
    token_issuer: TI,
}

impl<RTR, TR, SR, RTI, TI> DefaultRefreshTokenService<RTR, TR, SR, RTI, TI> {
    /// Creates a new refresh token service.
    #[must_use]
    pub const fn new(
        refresh_token_repository: RTR,
        token_repository: TR,
        session_repository: SR,
        refresh_token_issuer: RTI,
        token_issuer: TI
    ) -> Self {
        Self {
            refresh_token_repository,
            token_repository,
            session_repository,
            refresh_token_issuer,
            token_issuer,
        }
    }
}

fn hash_secret(secret: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(secret.as_bytes());

    hex::encode(hasher.finalize())
}

impl<RTR, TR, SR, RTI, TI> RefreshTokenService
    for DefaultRefreshTokenService<RTR, TR, SR, RTI, TI>
    where
        RTR: RefreshTokenRepository,
        TR: TokenRepository,
        SR: SessionRepository,
        RTI: RefreshTokenIssuer,
        TI: TokenIssuer
{
    type Error = AuthenticationError;

    fn refresh(&mut self, refresh_secret: &str) -> Result<RefreshResult, Self::Error> {
        let secret_hash = hash_secret(refresh_secret);

        let refresh_token = self.refresh_token_repository
            .find_by_secret_hash(&secret_hash)
            .map_err(|_| AuthenticationError::TokenRepositoryFailure)?
            .ok_or(AuthenticationError::TokenNotFound)?;

        let now = chrono::Utc::now();

        if !refresh_token.is_valid_at(now) {
            return Err(AuthenticationError::TokenUnavailable);
        }

        let session = self.session_repository
            .find_by_id(refresh_token.session_id())
            .map_err(|_| AuthenticationError::SessionRepositoryFailure)?
            .ok_or(AuthenticationError::SessionNotFound)?;

        if !session.is_valid_at(now) {
            return Err(AuthenticationError::SessionUnavailable);
        }

        let mut old_refresh_token = refresh_token;

        old_refresh_token.revoke();

        self.refresh_token_repository
            .save(old_refresh_token)
            .map_err(|_| AuthenticationError::TokenRepositoryFailure)?;

        let new_refresh_token = self.refresh_token_issuer
            .issue(session.id(), now + chrono::Duration::days(30))
            .map_err(|_| AuthenticationError::TokenCreationFailure)?;

        self.refresh_token_repository
            .save(new_refresh_token.token().clone())
            .map_err(|_| AuthenticationError::TokenRepositoryFailure)?;

        let access_token = self.token_issuer
            .issue(session.id(), now + chrono::Duration::hours(1))
            .map_err(|_| AuthenticationError::TokenCreationFailure)?;

        self.token_repository
            .save(access_token.token().clone())
            .map_err(|_| AuthenticationError::TokenRepositoryFailure)?;

        Ok(RefreshResult::new(access_token, new_refresh_token))
    }
}
