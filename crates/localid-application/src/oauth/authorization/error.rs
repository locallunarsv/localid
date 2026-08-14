/// OAuth authorization error.
#[derive(Debug)]
pub enum AuthorizationError {
    /// OAuth client is invalid.
    InvalidClient,

    /// Redirect URI is not registered.
    InvalidRedirectUri,

    /// Internal failure.
    InternalFailure,
}
