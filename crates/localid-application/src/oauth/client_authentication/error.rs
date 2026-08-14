/// OAuth client authentication error.
#[derive(Debug)]
pub enum ClientAuthenticationError {
    /// Client was not found.
    ClientNotFound,

    /// Client secret is invalid.
    InvalidSecret,

    /// OAuth client is inactive.
    ClientInactive,

    /// Repository failure.
    RepositoryFailure,
}
