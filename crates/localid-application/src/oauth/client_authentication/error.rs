/// OAuth client authentication error.
#[derive(Debug)]
pub enum ClientAuthenticationError {
    /// Client was not found.
    ClientNotFound,

    /// Client secret is invalid.
    InvalidSecret,

    /// Repository failure.
    RepositoryFailure,
}
