/// Client application errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientApplicationError {
    /// Client repository failure.
    RepositoryFailure,

    /// Client was not found.
    ClientNotFound,

    /// Client exists but cannot be used.
    ClientUnavailable,
}
