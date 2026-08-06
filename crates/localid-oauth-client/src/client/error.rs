/// OAuth client domain errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OAuthClientError {
    /// Client has already been deleted.
    AlreadyDeleted,
}
