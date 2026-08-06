use std::collections::HashMap;

use localid_oauth_authorization::{
    AuthorizationCode, AuthorizationCodeId, AuthorizationCodeRepository,
};

/// In-memory authorization code repository.
#[derive(Debug, Default, Clone)]
pub struct MemoryAuthorizationCodeRepository {
    codes: HashMap<AuthorizationCodeId, AuthorizationCode>,
}

impl MemoryAuthorizationCodeRepository {
    #[must_use]
    pub fn new() -> Self {
        Self {
            codes: HashMap::new(),
        }
    }
}

impl AuthorizationCodeRepository for MemoryAuthorizationCodeRepository {
    type Error = ();

    fn save(&mut self, code: AuthorizationCode) -> Result<(), Self::Error> {
        self.codes.insert(code.id(), code);

        Ok(())
    }

    fn find_by_id(
        &self,
        id: AuthorizationCodeId,
    ) -> Result<Option<AuthorizationCode>, Self::Error> {
        Ok(self.codes.get(&id).cloned())
    }
}
