use crate::ApplicationError;

use super::{IdentityLookupPort, ListIdentitiesResult};

/// Lists identities use case.
#[derive(Debug)]
pub struct ListIdentitiesUseCase<P> {
    port: P,
}

impl<P> ListIdentitiesUseCase<P> {
    /// Creates a new list identities use case.
    #[must_use]
    pub const fn new(port: P) -> Self {
        Self { port }
    }
}

impl<P> ListIdentitiesUseCase<P>
where
    P: IdentityLookupPort,
{
    /// Lists all identities.
    pub fn execute(&self) -> Result<ListIdentitiesResult, ApplicationError> {
        let identities = self
            .port
            .find_all_identities()
            .map_err(|_| ApplicationError::InternalFailure)?;

        Ok(ListIdentitiesResult::new(identities))
    }
}
