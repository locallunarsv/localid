use std::sync::{Arc, Mutex};

/// Shared repository wrapper.
#[derive(Debug)]
pub struct SharedRepository<T> {
    inner: Arc<Mutex<T>>,
}

impl<T> Clone for SharedRepository<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> SharedRepository<T> {
    /// Creates a shared repository.
    #[must_use]
    pub fn new(repository: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(repository)),
        }
    }

    /// Executes operation with immutable access.
    pub fn with<R>(&self, operation: impl FnOnce(&T) -> R) -> R {
        let repository = self.inner.lock().expect("repository lock poisoned");

        operation(&repository)
    }

    /// Executes operation with mutable access.
    pub fn with_mut<R>(&self, operation: impl FnOnce(&mut T) -> R) -> R {
        let mut repository = self.inner.lock().expect("repository lock poisoned");

        operation(&mut repository)
    }
}
