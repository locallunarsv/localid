use localid_identity::IdentityId;

/// Command for enabling an identity.
#[derive(Debug, Clone, Copy)]
pub struct EnableIdentityCommand {
    identity_id: IdentityId,
}

impl EnableIdentityCommand {
    /// Creates a new enable identity command.
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
