#![deny(missing_docs)]

//! In-memory repository implementations.

mod credential;
mod error;
mod identity;
mod identity_role;
mod password_material;
mod refresh_token;
mod session;
mod token;

pub use credential::MemoryCredentialRepository;
pub use error::MemoryRepositoryError;
pub use identity::MemoryIdentityRepository;
pub use password_material::MemoryPasswordMaterialRepository;
pub use refresh_token::MemoryRefreshTokenRepository;
pub use session::MemorySessionRepository;
pub use token::MemoryTokenRepository;

pub use identity_role::MemoryIdentityRoleRepository;
