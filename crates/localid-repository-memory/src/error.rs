use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// Generic in-memory repository error.
#[derive(Debug, Clone, Copy)]
pub struct MemoryRepositoryError;

impl Display for MemoryRepositoryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("memory repository error")
    }
}

impl Error for MemoryRepositoryError {}
