/// Application layer errors.
#[derive(Debug)]
pub enum ApplicationError {
    /// Authentication failed.
    AuthenticationFailed,

    /// Internal application failure.
    InternalFailure,
}
