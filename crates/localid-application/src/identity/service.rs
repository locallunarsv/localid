use localid_identity::IdentityId;

use crate::{ApplicationError, IdentityResult};

/// Identity lookup service abstraction.
pub trait IdentityLookupService {
    /// Finds identity by identifier.
    fn execute(&mut self, identity_id: IdentityId) -> Result<IdentityResult, ApplicationError>;
}
