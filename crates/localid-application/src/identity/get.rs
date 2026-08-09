use localid_identity::IdentityId;

use crate::ApplicationError;

use super::{IdentityLookupPort, IdentityLookupService, IdentityResult};

/// Identity lookup use case.
#[derive(Debug)]
pub struct GetIdentityUseCase<P> {
    port: P,
}

impl<P> GetIdentityUseCase<P> {
    /// Creates a new identity lookup use case.
    #[must_use]
    pub const fn new(port: P) -> Self {
        Self { port }
    }
}

impl<P> GetIdentityUseCase<P>
where
    P: IdentityLookupPort,
{
    /// Finds identity by identifier.
    pub fn execute(&mut self, identity_id: IdentityId) -> Result<IdentityResult, ApplicationError> {
        self.port
            .find_identity(identity_id)
            .map_err(|_| ApplicationError::InternalFailure)?
            .map(IdentityResult::new)
            .ok_or(ApplicationError::InternalFailure)
    }
}

impl<P> IdentityLookupService for GetIdentityUseCase<P>
where
    P: IdentityLookupPort,
{
    fn execute(&mut self, identity_id: IdentityId) -> Result<IdentityResult, ApplicationError> {
        GetIdentityUseCase::execute(self, identity_id)
    }
}
