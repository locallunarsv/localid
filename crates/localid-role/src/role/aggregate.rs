use localid_permission::Permission;

use super::RoleError;

/// Represents a group of permissions assigned to an identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Role {
    name: String,
    permissions: Vec<Permission>,
}

impl Role {
    /// Creates a new role.
    pub fn new(name: impl Into<String>, permissions: Vec<Permission>) -> Result<Self, RoleError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(RoleError::EmptyName);
        }

        Ok(Self { name, permissions })
    }

    /// Returns role name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns permissions owned by this role.
    #[must_use]
    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }

    /// Checks whether role contains a permission.
    #[must_use]
    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.iter().any(|item| item == permission)
    }
}
