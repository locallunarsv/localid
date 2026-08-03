use localid_identity::IdentityId;
use localid_repository::SessionRepository;
use localid_session::{Session, SessionId};

use super::SharedRepository;

impl<T> SessionRepository for SharedRepository<T>
where
    T: SessionRepository,
{
    type Error = T::Error;

    fn find_by_id(&self, id: SessionId) -> Result<Option<Session>, Self::Error> {
        self.with(|repository| repository.find_by_id(id))
    }

    fn find_by_identity_id(&self, identity_id: IdentityId) -> Result<Vec<Session>, Self::Error> {
        self.with(|repository| repository.find_by_identity_id(identity_id))
    }

    fn save(&mut self, session: Session) -> Result<(), Self::Error> {
        self.with_mut(|repository| repository.save(session))
    }
}
