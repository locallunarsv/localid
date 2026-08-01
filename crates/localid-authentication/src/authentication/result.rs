use localid_session::Session;

/// Successful authentication result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticateResult {
    session: Session,
}

impl AuthenticateResult {
    /// Creates a successful authentication result.
    #[must_use]
    pub const fn new(session: Session) -> Self {
        Self { session }
    }

    /// Returns the authenticated Session.
    #[must_use]
    pub const fn session(&self) -> &Session {
        &self.session
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeDelta, TimeZone, Utc};
    use localid_identity::IdentityId;
    use localid_session::{Session, SessionId};

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

    #[test]
    fn stores_authenticated_session() {
        let session = session();

        let result = AuthenticateResult::new(session.clone());

        assert_eq!(result.session(), &session);
    }
}
