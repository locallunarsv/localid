//! PKCE code challenge support.

/// PKCE code challenge method.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeChallengeMethod {
    /// SHA-256 based challenge.
    S256,
}

impl CodeChallengeMethod {
    /// Creates method from OAuth parameter value.
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "S256" => Some(Self::S256),
            _ => None,
        }
    }

    /// Returns OAuth parameter value.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::S256 => "S256",
        }
    }
}
