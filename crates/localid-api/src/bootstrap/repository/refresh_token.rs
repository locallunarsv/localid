use localid_refresh_token::{RefreshToken, RefreshTokenId};
use localid_repository::RefreshTokenRepository;

use super::SharedRepository;

impl<T> RefreshTokenRepository for SharedRepository<T>
where
    T: RefreshTokenRepository,
{
    type Error = T::Error;

    fn find_by_id(&self, id: RefreshTokenId) -> Result<Option<RefreshToken>, Self::Error> {
        self.with(|repository| repository.find_by_id(id))
    }

    fn find_by_secret_hash(&self, secret_hash: &str) -> Result<Option<RefreshToken>, Self::Error> {
        self.with(|repository| repository.find_by_secret_hash(secret_hash))
    }

    fn save(&mut self, token: RefreshToken) -> Result<(), Self::Error> {
        self.with_mut(|repository| repository.save(token))
    }
}
