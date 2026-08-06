//! Client application capabilities.

mod adapter;
mod error;
mod get;
mod port;
mod query;

pub use error::ClientApplicationError;
pub use port::ClientPort;
pub use query::FindClientQuery;

pub use adapter::ClientRepositoryAdapter;

pub use get::GetClientUseCase;
