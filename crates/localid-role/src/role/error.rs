/// Errors produced by role creation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoleError {
    /// Role name is empty.
    EmptyName,
}
