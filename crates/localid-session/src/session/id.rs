use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use uuid::Uuid;

/// Stable identifier for a [`Session`](crate::Session).
///
/// `SessionId` wraps the underlying identifier representation so consumers do
/// not need to use raw UUID values throughout the Session domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SessionId(Uuid);

impl SessionId {
    /// Generates a new Session identifier.
    ///
    /// The current implementation generates a UUID version 7.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Creates a Session identifier from an existing UUID.
    #[must_use]
    pub const fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    /// Returns a reference to the underlying UUID.
    #[must_use]
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Consumes this identifier and returns the underlying UUID.
    #[must_use]
    pub const fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for SessionId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

impl FromStr for SessionId {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(value).map(Self)
    }
}

impl From<Uuid> for SessionId {
    fn from(value: Uuid) -> Self {
        Self::from_uuid(value)
    }
}

impl From<SessionId> for Uuid {
    fn from(value: SessionId) -> Self {
        value.into_uuid()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::SessionId;

    #[test]
    fn creates_distinct_session_ids() {
        let first = SessionId::new();
        let second = SessionId::new();

        assert_ne!(first, second);
    }

    #[test]
    fn converts_session_id_to_string_and_back() {
        let original = SessionId::new();

        let parsed = SessionId::from_str(&original.to_string())
            .expect("generated SessionId should be parseable");

        assert_eq!(parsed, original);
    }

    #[test]
    fn rejects_invalid_session_id() {
        let result = SessionId::from_str("not-a-valid-uuid");

        assert!(result.is_err());
    }
}
