use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// Errors produced by the in-memory storage implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryStorageError {
    /// The shared storage lock was poisoned.
    LockPoisoned,
}

impl Display for MemoryStorageError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LockPoisoned => formatter.write_str("memory storage lock was poisoned"),
        }
    }
}

impl Error for MemoryStorageError {}
