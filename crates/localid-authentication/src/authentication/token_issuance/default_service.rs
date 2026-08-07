use chrono::Duration;

use localid_client::ClientId;
use localid_identity::IdentityId;
use localid_refresh_token_random::RefreshTokenIssuer;
use localid_repository::{RefreshTokenRepository, SessionRepository, TokenRepository};
use localid_token::TokenIssuer;

use crate::{AuthenticateResult, AuthenticationError, SessionFactory};

use super::TokenIssuanceService;

/// Default token issuance service.
///
/// Responsible for creating sessions and issuing access
/// and refresh tokens.
pub struct DefaultTokenIssuanceService<SR, TR, RTR, SF, TI, RTI> {
    session_repository: SR,
    token_repository: TR,
    refresh_token_repository: RTR,
    session_factory: SF,
    token_issuer: TI,
    refresh_token_issuer: RTI,
}

impl<SR, TR, RTR, SF, TI, RTI> DefaultTokenIssuanceService<SR, TR, RTR, SF, TI, RTI> {
    /// Creates a token issuance service.
    #[must_use]
    pub fn new(
        session_repository: SR,
        token_repository: TR,
        refresh_token_repository: RTR,
        session_factory: SF,
        token_issuer: TI,
        refresh_token_issuer: RTI,
    ) -> Self {
        Self {
            session_repository,
            token_repository,
            refresh_token_repository,
            session_factory,
            token_issuer,
            refresh_token_issuer,
        }
    }
}

impl<SR, TR, RTR, SF, TI, RTI> TokenIssuanceService
    for DefaultTokenIssuanceService<SR, TR, RTR, SF, TI, RTI>
where
    SR: SessionRepository,
    TR: TokenRepository,
    RTR: RefreshTokenRepository,
    SF: SessionFactory,
    TI: TokenIssuer<Error = localid_token::TokenError>,
    RTI: RefreshTokenIssuer<Error = localid_refresh_token::RefreshTokenError>,
{
    type Error = AuthenticationError;

    fn issue(
        &mut self,
        identity_id: IdentityId,
        client_id: ClientId,
    ) -> Result<AuthenticateResult, Self::Error> {
        let session = self
            .session_factory
            .create_session(identity_id, client_id)
            .map_err(|_| AuthenticationError::SessionCreationFailure)?;

        self.session_repository
            .save(session.clone())
            .map_err(|_| AuthenticationError::SessionRepositoryFailure)?;

        let issued_token = self
            .token_issuer
            .issue(session.id(), session.expires_at())
            .map_err(|_| AuthenticationError::TokenCreationFailure)?;

        self.token_repository
            .save(issued_token.token().clone())
            .map_err(|_| AuthenticationError::TokenRepositoryFailure)?;

        let issued_refresh_token = self
            .refresh_token_issuer
            .issue(session.id(), session.created_at() + Duration::days(30))
            .map_err(|_| AuthenticationError::TokenCreationFailure)?;

        self.refresh_token_repository
            .save(issued_refresh_token.token().clone())
            .map_err(|_| AuthenticationError::TokenRepositoryFailure)?;

        Ok(AuthenticateResult::new(
            session,
            issued_token,
            issued_refresh_token,
        ))
    }
}
