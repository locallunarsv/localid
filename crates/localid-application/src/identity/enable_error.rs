/// Errors returned when enabling an identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnableIdentityError {
    /// Identity was not found.
    NotFound,

    /// Identity has already been deleted.
    AlreadyDeleted,

    /// Repository operation failed.
    RepositoryFailure,
}
