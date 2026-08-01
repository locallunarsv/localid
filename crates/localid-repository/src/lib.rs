#![deny(missing_docs)]

//! Repository contracts for LocalID domain aggregates.

mod credential;
mod identity;
mod session;

pub use credential::CredentialRepository;
pub use identity::IdentityRepository;
pub use session::SessionRepository;
