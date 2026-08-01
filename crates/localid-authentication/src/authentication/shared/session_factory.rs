use localid_identity::IdentityId;
use localid_session::Session;

/// Creates Sessions after successful authentication.
///
/// Concrete implementations decide how creation time, expiration time, and
/// Session identifiers are produced.
pub trait SessionFactory {
    /// Error produced while creating a Session.
    type Error;

    /// Creates a new Session for an authenticated Identity.
    ///
    /// # Errors
    ///
    /// Returns the concrete factory error when a Session cannot be created.
    fn create_session(&self, identity_id: IdentityId) -> Result<Session, Self::Error>;
}
