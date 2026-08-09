use localid_identity::{Identity, IdentityId};
use localid_repository::IdentityRepository;

use super::SharedRepository;

impl<T> IdentityRepository for SharedRepository<T>
where
    T: IdentityRepository,
{
    type Error = T::Error;

    fn find_by_id(&self, id: IdentityId) -> Result<Option<Identity>, Self::Error> {
        self.with(|repository| repository.find_by_id(id))
    }

    fn save(&mut self, identity: Identity) -> Result<(), Self::Error> {
        self.with_mut(|repository| repository.save(identity))
    }
}
