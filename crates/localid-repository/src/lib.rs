#![deny(missing_docs)]

//! Repository contracts for LocalID domain aggregates.

mod credential;
mod identity;
mod password_material;
mod session;

pub use credential::CredentialRepository;
pub use identity::IdentityRepository;
pub use password_material::PasswordMaterialRepository;
pub use session::SessionRepository;
