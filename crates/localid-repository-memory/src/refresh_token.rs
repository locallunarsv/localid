use std::collections::HashMap;

use localid_refresh_token::{RefreshToken, RefreshTokenId};
use localid_repository::RefreshTokenRepository;

use crate::MemoryRepositoryError;

/// In-memory refresh token repository.
#[derive(Debug, Default)]
pub struct MemoryRefreshTokenRepository {
    tokens: HashMap<RefreshTokenId, RefreshToken>,
    secret_index: HashMap<String, RefreshTokenId>,
}

impl MemoryRefreshTokenRepository {
    /// Creates empty repository.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl RefreshTokenRepository for MemoryRefreshTokenRepository {
    type Error = MemoryRepositoryError;

    fn find_by_id(&self, id: RefreshTokenId) -> Result<Option<RefreshToken>, Self::Error> {
        Ok(self.tokens.get(&id).cloned())
    }

    fn find_by_secret_hash(&self, secret_hash: &str) -> Result<Option<RefreshToken>, Self::Error> {
        let Some(id) = self.secret_index.get(secret_hash) else {
            return Ok(None);
        };

        Ok(self.tokens.get(id).cloned())
    }

    fn save(&mut self, token: RefreshToken) -> Result<(), Self::Error> {
        self.secret_index
            .insert(token.secret_hash().to_owned(), token.id());

        self.tokens.insert(token.id(), token);

        Ok(())
    }
}
