use chrono::{DateTime, Utc};
use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha256};

use localid_refresh_token::{RefreshToken, RefreshTokenId};

use localid_session::SessionId;

/// Result of refresh token issuance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IssuedRefreshToken {
    token: RefreshToken,
    secret: String,
}

impl IssuedRefreshToken {
    /// Creates an issued refresh token.
    #[must_use]
    pub const fn new(token: RefreshToken, secret: String) -> Self {
        Self { token, secret }
    }

    /// Returns the stored Refresh Token aggregate.
    #[must_use]
    pub const fn token(&self) -> &RefreshToken {
        &self.token
    }

    /// Returns the raw secret.
    ///
    /// This value must only be shown once.
    #[must_use]
    pub fn secret(&self) -> &str {
        &self.secret
    }
}

/// Contract for Refresh Token issuance.
pub trait RefreshTokenIssuer {
    /// Error returned during issuance.
    type Error;

    /// Creates a new Refresh Token.
    fn issue(
        &self,
        session_id: SessionId,
        expires_at: DateTime<Utc>,
    ) -> Result<IssuedRefreshToken, Self::Error>;
}

/// Random Refresh Token issuer.
#[derive(Debug, Clone, Copy, Default)]
pub struct RandomRefreshTokenIssuer;

impl RandomRefreshTokenIssuer {
    /// Creates a new issuer.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    fn generate_secret() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect()
    }

    fn hash_secret(secret: &str) -> String {
        let mut hasher = Sha256::new();

        hasher.update(secret.as_bytes());

        hex::encode(hasher.finalize())
    }
}

impl RefreshTokenIssuer for RandomRefreshTokenIssuer {
    type Error = localid_refresh_token::RefreshTokenError;

    fn issue(
        &self,
        session_id: SessionId,
        expires_at: DateTime<Utc>,
    ) -> Result<IssuedRefreshToken, Self::Error> {
        let secret = Self::generate_secret();

        let token = RefreshToken::new(
            RefreshTokenId::new(),
            session_id,
            Self::hash_secret(&secret),
            Utc::now(),
            expires_at,
        )?;

        Ok(IssuedRefreshToken::new(token, secret))
    }
}
