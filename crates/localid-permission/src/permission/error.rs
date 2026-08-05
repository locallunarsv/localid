/// Errors produced by permission validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionError {
    /// Permission name is empty.
    EmptyName,
}
