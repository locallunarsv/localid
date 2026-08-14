/// Get OAuth client error.
#[derive(Debug)]
pub enum GetOAuthClientError {
    /// OAuth client does not exist.
    NotFound,

    /// Repository failure.
    RepositoryFailure,
}
