use super::Permission;

/// Represents a group of permissions assigned to an Identity.
///
/// A Role aggregates multiple permissions and provides a reusable
/// authorization boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Role {
    name: String,
    permissions: Vec<Permission>,
}

impl Role {
    /// Creates a new Role.
    #[must_use]
    pub fn new(name: impl Into<String>, permissions: Vec<Permission>) -> Self {
        Self {
            name: name.into(),
            permissions,
        }
    }

    /// Returns the role name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns all permissions assigned to this role.
    #[must_use]
    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }

    /// Returns true when this role contains the given permission.
    #[must_use]
    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission)
    }
}

#[cfg(test)]
mod tests {
    use super::Role;
    use crate::Permission;

    #[test]
    fn creates_role_with_permission() {
        let permission = Permission::new("user.read").expect("permission should be valid");

        let role = Role::new("reader", vec![permission.clone()]);

        assert_eq!(role.name(), "reader");
        assert!(role.has_permission(&permission));
    }
}
