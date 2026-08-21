use localid_identity::{Identity, IdentityId};
use localid_repository::IdentityRepository;

use crate::identity::IdentityLookupPort;

/// Adapter for identity repository.
pub struct IdentityRepositoryAdapter<R> {
    repository: R,
}

impl<R> IdentityRepositoryAdapter<R> {
    /// Creates a new identity repository adapter.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> IdentityLookupPort for IdentityRepositoryAdapter<R>
where
    R: IdentityRepository,
{
    type Error = R::Error;

    fn find_identity(&self, id: IdentityId) -> Result<Option<Identity>, Self::Error> {
        self.repository.find_by_id(id)
    }

    fn find_all_identities(&self) -> Result<Vec<Identity>, Self::Error> {
        self.repository.find_all()
    }
}
