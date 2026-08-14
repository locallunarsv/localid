/// Disable OAuth client error.
#[derive(Debug)]
pub enum DisableOAuthClientError {
    /// OAuth client was not found.
    NotFound,

    /// OAuth client is already deleted.
    AlreadyDeleted,

    /// Repository failure.
    RepositoryFailure,
}
