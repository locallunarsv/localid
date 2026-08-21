use localid_identity::{Identity, IdentityId};

/// Port for identity lookup.
pub trait IdentityLookupPort {
    /// Repository error.
    type Error;

    /// Finds identity by identifier.
    fn find_identity(&self, id: IdentityId) -> Result<Option<Identity>, Self::Error>;

    /// Finds all identities.
    fn find_all_identities(&self) -> Result<Vec<Identity>, Self::Error>;
}
