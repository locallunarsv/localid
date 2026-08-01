use localid_session::Session;
use localid_token::IssuedToken;

/// Successful authentication result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticateResult {
    session: Session,
    token: IssuedToken,
}

impl AuthenticateResult {
    /// Creates a successful authentication result.
    #[must_use]
    pub const fn new(session: Session, token: IssuedToken) -> Self {
        Self { session, token }
    }

    /// Returns the authenticated Session.
    #[must_use]
    pub const fn session(&self) -> &Session {
        &self.session
    }

    /// Returns the issued Token.
    #[must_use]
    pub const fn token(&self) -> &IssuedToken {
        &self.token
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeDelta, TimeZone, Utc};
    use localid_identity::IdentityId;
    use localid_session::{Session, SessionId};
    use localid_token::{Token, TokenId};

    use super::AuthenticateResult;

    fn session() -> Session {
        let created_at = Utc.with_ymd_and_hms(2026, 8, 2, 0, 0, 0).single().unwrap();

        Session::new(
            SessionId::new(),
            IdentityId::new(),
            created_at,
            created_at + TimeDelta::hours(1),
        )
        .unwrap()
    }

    fn issued_token(session: &Session) -> localid_token::IssuedToken {
        let token = Token::new(
            TokenId::new(),
            session.id(),
            "hashed-secret".to_owned(),
            session.created_at(),
            session.expires_at(),
        )
        .unwrap();

        localid_token::IssuedToken::new(token, "raw-secret".to_owned())
    }

    #[test]
    fn stores_authenticated_session() {
        let session = session();
        let token = issued_token(&session);

        let result = AuthenticateResult::new(session.clone(), token.clone());

        assert_eq!(result.session(), &session);
        assert_eq!(result.token(), &token);
    }
}
