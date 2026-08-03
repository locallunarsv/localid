use std::collections::HashMap;

use localid_repository::TokenRepository;
use localid_token::{Token, TokenId};

use crate::MemoryRepositoryError;

/// In-memory token repository.
#[derive(Debug, Default)]
pub struct MemoryTokenRepository {
    tokens: HashMap<TokenId, Token>,
    secret_index: HashMap<String, TokenId>,
}

impl MemoryTokenRepository {
    /// Creates empty repository.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl TokenRepository for MemoryTokenRepository {
    type Error = MemoryRepositoryError;

    fn find_by_id(&self, id: TokenId) -> Result<Option<Token>, Self::Error> {
        Ok(self.tokens.get(&id).cloned())
    }

    fn find_by_secret_hash(&self, secret_hash: &str) -> Result<Option<Token>, Self::Error> {
        let Some(id) = self.secret_index.get(secret_hash) else {
            return Ok(None);
        };

        Ok(self.tokens.get(id).cloned())
    }

    fn save(&mut self, token: Token) -> Result<(), Self::Error> {
        self.secret_index
            .insert(token.secret_hash().to_owned(), token.id());

        self.tokens.insert(token.id(), token);

        Ok(())
    }
}
