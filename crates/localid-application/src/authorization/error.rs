/// Errors produced during authorization preparation.
#[derive(Debug)]
pub enum AuthorizationApplicationError {
    /// Failed to resolve identity roles.
    RoleResolutionFailure,
}
