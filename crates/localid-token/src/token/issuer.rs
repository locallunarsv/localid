use chrono::{DateTime, Utc};
use localid_session::SessionId;

use super::Token;

/// Result of issuing a new Token.
///
/// The raw secret is returned once and must not be stored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IssuedToken {
    token: Token,
    secret: String,
}

impl IssuedToken {
    /// Creates a new issued token result.
    #[must_use]
    pub const fn new(token: Token, secret: String) -> Self {
        Self { token, secret }
    }

    /// Returns the persisted Token entity.
    #[must_use]
    pub const fn token(&self) -> &Token {
        &self.token
    }

    /// Returns the raw token secret.
    ///
    /// The caller is responsible for delivering it securely.
    #[must_use]
    pub fn secret(&self) -> &str {
        &self.secret
    }
}

/// Contract for generating Token values.
pub trait TokenIssuer {
    /// Error returned during token creation.
    type Error;

    /// Creates a Token for a Session.
    fn issue(
        &self,
        session_id: SessionId,
        expires_at: DateTime<Utc>,
    ) -> Result<IssuedToken, Self::Error>;
}
