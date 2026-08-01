use chrono::{DateTime, Utc};
use localid_session::SessionId;
use localid_token::{IssuedToken, Token, TokenError, TokenId, TokenIssuer};
use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha256};

/// Random token issuer implementation.
#[derive(Debug, Default, Clone, Copy)]
pub struct RandomTokenIssuer;

impl RandomTokenIssuer {
    /// Creates a new random token issuer.
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

impl TokenIssuer for RandomTokenIssuer {
    type Error = TokenError;

    fn issue(
        &self,
        session_id: SessionId,
        expires_at: DateTime<Utc>,
    ) -> Result<IssuedToken, Self::Error> {
        let secret = Self::generate_secret();
        let secret_hash = Self::hash_secret(&secret);

        let created_at = Utc::now();

        let token = Token::new(
            TokenId::new(),
            session_id,
            secret_hash,
            created_at,
            expires_at,
        )?;

        Ok(IssuedToken::new(token, secret))
    }
}
