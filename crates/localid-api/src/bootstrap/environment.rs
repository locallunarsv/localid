/// Runtime environment mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
