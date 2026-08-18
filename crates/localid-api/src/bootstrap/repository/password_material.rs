use localid_credential::CredentialId;
use localid_password::PasswordMaterial;
use localid_repository::PasswordMaterialRepository;

use super::SharedRepository;

impl<T> PasswordMaterialRepository for SharedRepository<T>
where
    T: PasswordMaterialRepository,
{
    type Error = T::Error;

    fn find_by_credential_id(
        &self,
        credential_id: CredentialId,
    ) -> Result<Option<PasswordMaterial>, Self::Error> {
        self.with(|repository| repository.find_by_credential_id(credential_id))
    }

    fn save(&mut self, material: PasswordMaterial) -> Result<(), Self::Error> {
        self.with_mut(|repository| repository.save(material))
    }
}
