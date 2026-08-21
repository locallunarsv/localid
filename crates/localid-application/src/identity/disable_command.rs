use localid_identity::IdentityId;

/// Command for disabling an identity.
#[derive(Debug, Clone, Copy)]
pub struct DisableIdentityCommand {
    identity_id: IdentityId,
}

impl DisableIdentityCommand {
    /// Creates a new disable identity command.
    #[must_use]
    pub const fn new(identity_id: IdentityId) -> Self {
        Self { identity_id }
    }

    /// Returns identity identifier.
    #[must_use]
    pub const fn identity_id(&self) -> IdentityId {
        self.identity_id
    }
}
