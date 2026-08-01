use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use uuid::Uuid;

/// Stable identifier for an [`Identity`](crate::Identity).
///
/// `IdentityId` wraps the underlying identifier representation so consumers
/// of this crate do not need to use raw UUID values throughout the domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IdentityId(Uuid);

impl IdentityId {
    /// Generates a new identity identifier.
    ///
    /// The current implementation generates a UUID version 7.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Creates an identity identifier from an existing UUID.
    #[must_use]
    pub const fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    /// Returns a reference to the underlying UUID.
    #[must_use]
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Consumes the identity identifier and returns the underlying UUID.
    #[must_use]
    pub const fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for IdentityId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for IdentityId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

impl FromStr for IdentityId {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(value).map(Self)
    }
}

impl From<Uuid> for IdentityId {
    fn from(value: Uuid) -> Self {
        Self::from_uuid(value)
    }
}

impl From<IdentityId> for Uuid {
    fn from(value: IdentityId) -> Self {
        value.into_uuid()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::IdentityId;

    #[test]
    fn creates_distinct_identity_ids() {
        let first = IdentityId::new();
        let second = IdentityId::new();

        assert_ne!(first, second);
    }

    #[test]
    fn converts_identity_id_to_string_and_back() {
        let original = IdentityId::new();

        let parsed = IdentityId::from_str(&original.to_string())
            .expect("generated IdentityId should be parseable");

        assert_eq!(parsed, original);
    }

    #[test]
    fn rejects_invalid_identity_id() {
        let result = IdentityId::from_str("not-a-valid-uuid");

        assert!(result.is_err());
    }
}
