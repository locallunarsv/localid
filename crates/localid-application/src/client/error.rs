/// Client application errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientApplicationError {
    /// Client repository failure.
    RepositoryFailure,

    /// Client was not found.
    ClientNotFound,
}
