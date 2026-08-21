/// Errors returned when deleting an identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteIdentityError {
    /// Identity was not found.
    NotFound,

    /// Repository operation failed.
    RepositoryFailure,
}
