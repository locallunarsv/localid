/// Errors returned during authorization domain validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationError {
    /// Permission name is empty.
    EmptyPermissionName,

    /// Permission name does not follow namespace.action format.
    InvalidPermissionFormat,
}
