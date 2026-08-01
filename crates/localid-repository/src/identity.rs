use localid_identity::{Identity, IdentityId};

/// Repository for Identity aggregates.
pub trait IdentityRepository {
    /// Repository-specific error.
    type Error;

    /// Finds an Identity by its identifier.
    fn find_by_id(&self, id: IdentityId) -> Result<Option<Identity>, Self::Error>;

    /// Persists an Identity.
    fn save(&mut self, identity: Identity) -> Result<(), Self::Error>;
}
