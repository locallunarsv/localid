use localid_oauth_authorization::{
    AuthorizationCode, AuthorizationCodeId, AuthorizationCodeRepository,
};

use super::SharedRepository;

use localid_oauth_authorization_repository_memory::MemoryAuthorizationCodeRepository;

impl AuthorizationCodeRepository for SharedRepository<MemoryAuthorizationCodeRepository> {
    type Error = ();

    fn save(&mut self, code: AuthorizationCode) -> Result<(), Self::Error> {
        self.with_mut(|repository| repository.save(code))
    }

    fn find_by_id(
        &self,
        id: AuthorizationCodeId,
    ) -> Result<Option<AuthorizationCode>, Self::Error> {
        self.with(|repository| repository.find_by_id(id))
    }
}
