use localid_identity::Identity;

/// Identity lookup result.
#[derive(Debug, Clone)]
pub struct IdentityResult {
    identity: Identity,
}

impl IdentityResult {
    /// Creates identity result.
    #[must_use]
    pub const fn new(identity: Identity) -> Self {
        Self { identity }
    }

    /// Returns identity aggregate.
    #[must_use]
    pub const fn identity(&self) -> &Identity {
        &self.identity
    }
}
