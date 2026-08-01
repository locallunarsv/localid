use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use uuid::Uuid;

/// Stable identifier for a [`Credential`](crate::Credential).
///
/// `CredentialId` wraps the underlying identifier representation so consumers
/// do not need to use raw UUID values throughout the Credential domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CredentialId(Uuid);

impl CredentialId {
    /// Generates a new Credential identifier.
    ///
    /// The current implementation generates a UUID version 7.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Creates a Credential identifier from an existing UUID.
    #[must_use]
    pub const fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    /// Returns a reference to the underlying UUID.
    #[must_use]
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Consumes the Credential identifier and returns the underlying UUID.
    #[must_use]
    pub const fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for CredentialId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for CredentialId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

impl FromStr for CredentialId {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(value).map(Self)
    }
}

impl From<Uuid> for CredentialId {
    fn from(value: Uuid) -> Self {
        Self::from_uuid(value)
    }
}

impl From<CredentialId> for Uuid {
    fn from(value: CredentialId) -> Self {
        value.into_uuid()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::CredentialId;

    #[test]
    fn creates_distinct_credential_ids() {
        let first = CredentialId::new();
        let second = CredentialId::new();

        assert_ne!(first, second);
    }

    #[test]
    fn converts_credential_id_to_string_and_back() {
        let original = CredentialId::new();

        let parsed = CredentialId::from_str(&original.to_string())
            .expect("generated CredentialId should be parseable");

        assert_eq!(parsed, original);
    }

    #[test]
    fn rejects_invalid_credential_id() {
        let result = CredentialId::from_str("not-a-valid-uuid");

        assert!(result.is_err());
    }
}
