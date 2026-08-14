/// Activate OAuth client error.
#[derive(Debug)]
pub enum ActivateOAuthClientError {
    /// OAuth client was not found.
    NotFound,

    /// OAuth client has already been deleted.
    AlreadyDeleted,

    /// Repository failure.
    RepositoryFailure,
}
