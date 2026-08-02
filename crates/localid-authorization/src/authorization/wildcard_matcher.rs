use super::{Permission, PermissionMatcher};

/// Permission matcher supporting trailing wildcard.
///
/// Example:
/// `user.*` matches `user.read` and `user.delete`.
#[derive(Debug, Clone, Copy, Default)]
pub struct WildcardPermissionMatcher;

impl WildcardPermissionMatcher {
    /// Creates a new wildcard permission matcher.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl PermissionMatcher for WildcardPermissionMatcher {
    fn matches(&self, granted: &Permission, requested: &Permission) -> bool {
        let granted_name = granted.name();
        let requested_name = requested.name();

        if granted_name == requested_name {
            return true;
        }

        if let Some(prefix) = granted_name.strip_suffix(".*") {
            return requested_name.starts_with(prefix);
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::{PermissionMatcher, WildcardPermissionMatcher};
    use crate::Permission;

    #[test]
    fn matches_permission_with_wildcard() {
        let matcher = WildcardPermissionMatcher::new();

        let granted = Permission::new("user.*").expect("permission should be valid");

        let requested = Permission::new("user.read").expect("permission should be valid");

        assert!(matcher.matches(&granted, &requested));
    }

    #[test]
    fn rejects_different_namespace() {
        let matcher = WildcardPermissionMatcher::new();

        let granted = Permission::new("user.*").expect("permission should be valid");

        let requested = Permission::new("admin.read").expect("permission should be valid");

        assert!(!matcher.matches(&granted, &requested));
    }

    #[test]
    fn matches_exact_permission() {
        let matcher = WildcardPermissionMatcher::new();

        let granted = Permission::new("user.read").expect("permission should be valid");

        let requested = Permission::new("user.read").expect("permission should be valid");

        assert!(matcher.matches(&granted, &requested));
    }
}
