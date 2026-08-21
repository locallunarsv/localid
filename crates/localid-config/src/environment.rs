//! Runtime environment configuration.

use serde::Deserialize;

/// Runtime environment mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    /// Development mode with demo data.
    Development,

    /// Production mode without demo seed.
    Production,
}

impl Environment {
    /// Returns whether demo seed should be created.
    #[must_use]
    pub const fn should_seed(self) -> bool {
        matches!(self, Self::Development)
    }
}
