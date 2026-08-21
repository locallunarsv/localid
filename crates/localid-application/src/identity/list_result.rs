use localid_identity::Identity;

/// Result returned from listing identities.
#[derive(Debug, Clone)]
pub struct ListIdentitiesResult {
    identities: Vec<Identity>,
}

impl ListIdentitiesResult {
    /// Creates list identities result.
    #[must_use]
    pub const fn new(identities: Vec<Identity>) -> Self {
        Self { identities }
    }

    /// Returns identities.
    #[must_use]
    pub fn identities(&self) -> &[Identity] {
        &self.identities
    }
}
