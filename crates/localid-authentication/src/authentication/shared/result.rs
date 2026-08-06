use localid_refresh_token_random::IssuedRefreshToken;
use localid_session::Session;
use localid_token_random::IssuedToken;

/// Successful authentication result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticateResult {
    session: Session,
    token: IssuedToken,
    refresh_token: IssuedRefreshToken,
}

impl AuthenticateResult {
    /// Creates a successful authentication result.
    #[must_use]
    pub const fn new(
        session: Session,
        token: IssuedToken,
        refresh_token: IssuedRefreshToken,
    ) -> Self {
        Self {
            session,
            token,
            refresh_token,
        }
    }

    /// Returns the authenticated session.
    #[must_use]
    pub const fn session(&self) -> &Session {
        &self.session
    }

    /// Returns the access token.
    #[must_use]
    pub const fn token(&self) -> &IssuedToken {
        &self.token
    }

    /// Returns the refresh token.
    #[must_use]
    pub const fn refresh_token(&self) -> &IssuedRefreshToken {
        &self.refresh_token
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeDelta, TimeZone, Utc};
    use localid_client::ClientId;
    use localid_identity::IdentityId;
    use localid_refresh_token::RefreshToken;
    use localid_refresh_token_random::IssuedRefreshToken;
    use localid_session::{Session, SessionId};
    use localid_token::Token;
    use localid_token_random::IssuedToken;

    use super::AuthenticateResult;

    fn session() -> Session {
        let created_at = Utc.with_ymd_and_hms(2026, 8, 2, 0, 0, 0).single().unwrap();

        Session::new(
            SessionId::new(),
            IdentityId::new(),
            ClientId::new(),
            created_at,
            created_at + TimeDelta::hours(1),
        )
        .unwrap()
    }

    #[test]
    fn stores_authenticated_session() {
        let session = session();

        let token = IssuedToken::new(
            Token::new(
                localid_token::TokenId::new(),
                session.id(),
                "hash".to_owned(),
                session.created_at(),
                session.expires_at(),
            )
            .unwrap(),
            "secret".to_owned(),
        );

        let refresh_token = IssuedRefreshToken::new(
            RefreshToken::new(
                localid_refresh_token::RefreshTokenId::new(),
                session.id(),
                "refresh-hash".to_owned(),
                session.created_at(),
                session.expires_at(),
            )
            .unwrap(),
            "refresh-secret".to_owned(),
        );

        let result = AuthenticateResult::new(session.clone(), token, refresh_token);

        assert_eq!(result.session(), &session);
        assert_eq!(result.session().client_id(), session.client_id());
    }
}
