/// Errors produced by client aggregate operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientError {
    /// Client is already deleted.
    AlreadyDeleted,

    /// Client operation is not allowed.
    InvalidState,
}
