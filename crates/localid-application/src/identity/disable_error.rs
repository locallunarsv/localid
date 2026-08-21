/// Errors returned when disabling an identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisableIdentityError {
    /// Identity was not found.
    NotFound,

    /// Identity has already been deleted.
    AlreadyDeleted,

    /// Repository operation failed.
    RepositoryFailure,
}
