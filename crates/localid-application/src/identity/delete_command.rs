use localid_identity::IdentityId;

/// Command for deleting an identity.
#[derive(Debug, Clone, Copy)]
pub struct DeleteIdentityCommand {
    identity_id: IdentityId,
}

impl DeleteIdentityCommand {
    /// Creates a new delete identity command.
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
