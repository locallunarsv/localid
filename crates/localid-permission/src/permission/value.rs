use super::PermissionError;

/// Represents an authorization permission.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Permission {
    name: String,
}

impl Permission {
    /// Creates a new permission.
    pub fn new(name: impl Into<String>) -> Result<Self, PermissionError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(PermissionError::EmptyName);
        }

        Ok(Self { name })
    }

    /// Returns permission name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}
