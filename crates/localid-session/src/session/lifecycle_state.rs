/// Lifecycle state of a Session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionLifecycleState {
    /// Session remains operational until revoked or expired.
    Active,

    /// Session has been explicitly revoked.
    Revoked,
}
