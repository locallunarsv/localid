/// Delete OAuth client error.
#[derive(Debug)]
pub enum DeleteOAuthClientError {
    /// OAuth client was not found.
    NotFound,

    /// OAuth client was already deleted.
    AlreadyDeleted,

    /// Repository failure.
    RepositoryFailure,
}
