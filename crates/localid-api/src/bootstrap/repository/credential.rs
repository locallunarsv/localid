use localid_credential::{Credential, CredentialId};
use localid_identity::IdentityId;
use localid_repository::CredentialRepository;

use super::SharedRepository;

impl<T> CredentialRepository for SharedRepository<T>
where
    T: CredentialRepository,
{
    type Error = T::Error;

    fn find_by_id(&self, id: CredentialId) -> Result<Option<Credential>, Self::Error> {
        self.with(|repository| repository.find_by_id(id))
    }

    fn find_by_identity_id(&self, identity_id: IdentityId) -> Result<Vec<Credential>, Self::Error> {
        self.with(|repository| repository.find_by_identity_id(identity_id))
    }

    fn save(&mut self, credential: Credential) -> Result<(), Self::Error> {
        self.with_mut(|repository| repository.save(credential))
    }
}
