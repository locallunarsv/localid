use super::Permission;

/// Permission matching strategy.
///
/// Responsible for deciding whether a granted permission
/// satisfies a requested permission.
pub trait PermissionMatcher {
    /// Returns true when granted permission matches requested permission.
    fn matches(&self, granted: &Permission, requested: &Permission) -> bool;
}

/// Default exact permission matcher.
#[derive(Debug, Clone, Copy, Default)]
pub struct ExactPermissionMatcher;

impl ExactPermissionMatcher {
    /// Creates a new exact matcher.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl PermissionMatcher for ExactPermissionMatcher {
    fn matches(&self, granted: &Permission, requested: &Permission) -> bool {
        granted == requested
    }
}

#[cfg(test)]
mod tests {
    use super::{ExactPermissionMatcher, PermissionMatcher};
    use crate::Permission;

    #[test]
    fn matches_same_permission() {
        let matcher = ExactPermissionMatcher::new();

        let granted = Permission::new("user.read").expect("permission should be valid");

        let requested = Permission::new("user.read").expect("permission should be valid");

        assert!(matcher.matches(&granted, &requested));
    }

    #[test]
    fn rejects_different_permission() {
        let matcher = ExactPermissionMatcher::new();

        let granted = Permission::new("user.read").expect("permission should be valid");

        let requested = Permission::new("user.delete").expect("permission should be valid");

        assert!(!matcher.matches(&granted, &requested));
    }
}
