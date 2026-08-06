/// Authorization code domain errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationCodeError {
    /// Expiration time is invalid.
    InvalidExpirationTime,

    /// Authorization code already consumed.
    AlreadyConsumed,
}
