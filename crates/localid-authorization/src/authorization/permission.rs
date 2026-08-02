use super::AuthorizationError;

/// Represents an authorization permission.
///
/// Permission format:
/// `namespace.action`
///
/// Example:
/// `user.read`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Permission {
    name: String,
}

impl Permission {
    /// Creates a new permission.
    ///
    /// The permission name is normalized by trimming
    /// leading and trailing whitespace before validation.
    ///
    /// # Errors
    ///
    /// Returns [`AuthorizationError`] when the permission format is invalid.
    pub fn new(name: impl Into<String>) -> Result<Self, AuthorizationError> {
        let name = name.into().trim().to_owned();

        if name.is_empty() {
            return Err(AuthorizationError::EmptyPermissionName);
        }

        let mut parts = name.split('.');

        let namespace = parts.next();
        let action = parts.next();

        if namespace.is_none()
            || action.is_none()
            || parts.next().is_some()
            || namespace.unwrap().is_empty()
            || action.unwrap().is_empty()
            || name.chars().any(|c| c.is_uppercase())
        {
            return Err(AuthorizationError::InvalidPermissionFormat);
        }

        Ok(Self { name })
    }

    /// Returns permission name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::{AuthorizationError, Permission};

    #[test]
    fn rejects_empty_permission_name() {
        let result = Permission::new("");

        assert_eq!(result, Err(AuthorizationError::EmptyPermissionName));
    }

    #[test]
    fn rejects_uppercase_permission_name() {
        let result = Permission::new("User.Read");

        assert_eq!(result, Err(AuthorizationError::InvalidPermissionFormat));
    }

    #[test]
    fn rejects_permission_without_action() {
        let result = Permission::new("user");

        assert_eq!(result, Err(AuthorizationError::InvalidPermissionFormat));
    }

    #[test]
    fn trims_permission_name() {
        let permission = Permission::new(" user.read ").expect("permission should be valid");

        assert_eq!(permission.name(), "user.read");
    }
}
