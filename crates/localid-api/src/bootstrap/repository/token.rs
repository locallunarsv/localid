use localid_repository::TokenRepository;
use localid_token::{Token, TokenId};

use super::SharedRepository;

impl<T> TokenRepository for SharedRepository<T>
where
    T: TokenRepository,
{
    type Error = T::Error;

    fn find_by_id(&self, id: TokenId) -> Result<Option<Token>, Self::Error> {
        self.with(|repository| repository.find_by_id(id))
    }

    fn find_by_secret_hash(&self, secret_hash: &str) -> Result<Option<Token>, Self::Error> {
        self.with(|repository| repository.find_by_secret_hash(secret_hash))
    }

    fn save(&mut self, token: Token) -> Result<(), Self::Error> {
        self.with_mut(|repository| repository.save(token))
    }
}
