#![deny(missing_docs)]

//! Repository contracts for LocalID domain aggregates.

mod credential;
mod identity;
mod identity_role;
mod password_material;
mod refresh_token;
mod session;
mod token;

pub use credential::CredentialRepository;
pub use identity::IdentityRepository;
pub use password_material::PasswordMaterialRepository;
pub use refresh_token::RefreshTokenRepository;
pub use session::SessionRepository;
pub use token::TokenRepository;

pub use identity_role::IdentityRoleRepository;
